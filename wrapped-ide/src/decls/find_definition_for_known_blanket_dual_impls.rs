macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! find_definition_for_known_blanket_dual_impls {
    () => {
        deps!();
        fn find_definition_for_known_blanket_dual_impls (sema : & Semantics < '_ , RootDatabase > , original_token : & SyntaxToken ,) -> Option < Vec < NavigationTarget > > { let method_call = ast :: MethodCallExpr :: cast (original_token . parent () ? . parent () ?) ? ; let callable = sema . resolve_method_call_as_callable (& method_call) ? ; let CallableKind :: Function (f) = callable . kind () else { return None } ; let assoc = f . as_assoc_item (sema . db) ? ; let return_type = callable . return_type () ; let fd = FamousDefs (sema , return_type . krate (sema . db)) ; let t = match assoc . container (sema . db) { hir :: AssocItemContainer :: Trait (t) => t , hir :: AssocItemContainer :: Impl (impl_) if impl_ . self_ty (sema . db) . is_str () && f . name (sema . db) == sym :: parse => { let t = fd . core_convert_FromStr () ? ; let t_f = t . function (sema . db , & sym :: from_str) ? ; return sema . resolve_trait_impl_method (return_type . clone () , t , t_f , [return_type . type_arguments () . next () ?] ,) . map (| f | def_to_nav (sema , f . into ())) ; } hir :: AssocItemContainer :: Impl (_) => return None , } ; let fn_name = f . name (sema . db) ; let f = if fn_name == sym :: into && fd . core_convert_Into () == Some (t) { let dual = fd . core_convert_From () ? ; let dual_f = dual . function (sema . db , & sym :: from) ? ; sema . resolve_trait_impl_method (return_type . clone () , dual , dual_f , [return_type , callable . receiver_param (sema . db) ? . 1] ,) ? } else if fn_name == sym :: try_into && fd . core_convert_TryInto () == Some (t) { let dual = fd . core_convert_TryFrom () ? ; let dual_f = dual . function (sema . db , & sym :: try_from) ? ; sema . resolve_trait_impl_method (return_type . clone () , dual , dual_f , [return_type . type_arguments () . next () ? , callable . receiver_param (sema . db) ? . 1] ,) ? } else if fn_name == sym :: to_string && fd . alloc_string_ToString () == Some (t) { let dual = fd . core_fmt_Display () ? ; let dual_f = dual . function (sema . db , & sym :: fmt) ? ; sema . resolve_trait_impl_method (return_type . clone () , dual , dual_f , [callable . receiver_param (sema . db) ? . 1 . strip_reference ()] ,) ? } else { return None ; } ; let _t = f . as_assoc_item (sema . db) ? . implemented_trait (sema . db) ? ; let def = Definition :: from (f) ; Some (def_to_nav (sema , def)) }
    };
}

find_definition_for_known_blanket_dual_impls!()