macro_rules! deps {
    () => {
        HirTyLowerer!();
    };
}

macro_rules! placeholder_type_error_diag {
    () => {
        deps!();
        fn placeholder_type_error_diag < 'cx , 'tcx > (cx : & 'cx dyn HirTyLowerer < 'tcx > , generics : Option < & hir :: Generics < '_ > > , placeholder_types : Vec < Span > , additional_spans : Vec < Span > , suggest : bool , hir_ty : Option < & hir :: Ty < '_ > > , kind : & 'static str ,) -> Diag < 'cx > { if placeholder_types . is_empty () { return bad_placeholder (cx , additional_spans , kind) ; } let params = generics . map (| g | g . params) . unwrap_or_default () ; let type_name = params . next_type_param_name (None) ; let mut sugg : Vec < _ > = placeholder_types . iter () . map (| sp | (* sp , (* type_name) . to_string ())) . collect () ; if let Some (generics) = generics { if let Some (span) = params . iter () . find_map (| arg | match arg . name { hir :: ParamName :: Plain (Ident { name : kw :: Underscore , span }) => Some (span) , _ => None , }) { sugg . push ((span , (* type_name) . to_string ())) ; } else if let Some (span) = generics . span_for_param_suggestion () { sugg . push ((span , format ! (", {type_name}"))) ; } else { sugg . push ((generics . span , format ! ("<{type_name}>"))) ; } } let mut err = bad_placeholder (cx , placeholder_types . into_iter () . chain (additional_spans) . collect () , kind) ; if suggest { let mut is_fn = false ; let mut is_const_or_static = false ; if let Some (hir_ty) = hir_ty && let hir :: TyKind :: FnPtr (_) = hir_ty . kind { is_fn = true ; is_const_or_static = matches ! (cx . tcx () . parent_hir_node (hir_ty . hir_id) , Node :: Item (& hir :: Item { kind : hir :: ItemKind :: Const (..) | hir :: ItemKind :: Static (..) , .. }) | Node :: TraitItem (& hir :: TraitItem { kind : hir :: TraitItemKind :: Const (..) , .. }) | Node :: ImplItem (& hir :: ImplItem { kind : hir :: ImplItemKind :: Const (..) , .. })) ; } if ! (is_fn && is_const_or_static) { err . multipart_suggestion ("use type parameters instead" , sugg , Applicability :: HasPlaceholders ,) ; } } err }
    };
}

placeholder_type_error_diag!()