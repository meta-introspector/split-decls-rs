macro_rules! deps {
    () => {
        SelfSource!();
    };
}

macro_rules! print_disambiguation_help {
    () => {
        deps!();
        fn print_disambiguation_help < 'tcx > (tcx : TyCtxt < 'tcx > , err : & mut Diag < '_ > , source : SelfSource < 'tcx > , args : Option < & 'tcx [hir :: Expr < 'tcx >] > , trait_ref : ty :: TraitRef < 'tcx > , candidate_idx : Option < usize > , span : Span , item : ty :: AssocItem ,) -> Option < String > { let trait_impl_type = trait_ref . self_ty () . peel_refs () ; let trait_ref = if item . is_method () { trait_ref . print_only_trait_name () . to_string () } else { format ! ("<{} as {}>" , trait_ref . args [0] , trait_ref . print_only_trait_name ()) } ; Some (if item . is_fn () && let SelfSource :: MethodCall (receiver) = source && let Some (args) = args { let def_kind_descr = tcx . def_kind_descr (item . as_def_kind () , item . def_id) ; let item_name = item . ident (tcx) ; let first_input = tcx . fn_sig (item . def_id) . instantiate_identity () . skip_binder () . inputs () . get (0) ; let (first_arg_type , rcvr_ref) = (first_input . map (| first | first . peel_refs ()) , first_input . and_then (| ty | ty . ref_mutability ()) . map_or ("" , | mutbl | mutbl . ref_prefix_str ()) ,) ; let args = if let Some (first_arg_type) = first_arg_type && (first_arg_type == tcx . types . self_param || first_arg_type == trait_impl_type || item . is_method ()) { Some (receiver) } else { None } . into_iter () . chain (args) . map (| arg | { tcx . sess . source_map () . span_to_snippet (arg . span) . unwrap_or_else (| _ | "_" . to_owned ()) }) . collect :: < Vec < _ > > () . join (", ") ; let args = format ! ("({}{})" , rcvr_ref , args) ; err . span_suggestion_verbose (span , format ! ("disambiguate the {def_kind_descr} for {}" , if let Some (candidate) = candidate_idx { format ! ("candidate #{candidate}") } else { "the candidate" . to_string () } ,) , format ! ("{trait_ref}::{item_name}{args}") , Applicability :: HasPlaceholders ,) ; return None ; } else { format ! ("{trait_ref}::") } ,) }
    };
}

print_disambiguation_help!()