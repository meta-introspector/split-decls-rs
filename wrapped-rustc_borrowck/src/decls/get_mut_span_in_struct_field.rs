macro_rules! get_mut_span_in_struct_field {
    () => {
        # [doc = " Given a field that needs to be mutable, returns a span where the \" mut \" could go."] # [doc = " This function expects the local to be a reference to a struct in order to produce a span."] # [doc = ""] # [doc = " ```text"] # [doc = " LL |     s: &'a   String"] # [doc = "    |           ^^^ returns a span taking up the space here"] # [doc = " ```"] fn get_mut_span_in_struct_field < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , field : FieldIdx ,) -> Option < Span > { if let ty :: Ref (_ , ty , _) = ty . kind () && let ty :: Adt (def , _) = ty . kind () && let field = def . all_fields () . nth (field . index ()) ? && let hir :: Node :: Field (field) = tcx . hir_node_by_def_id (field . did . as_local () ?) && let hir :: TyKind :: Ref (lt , hir :: MutTy { mutbl : hir :: Mutability :: Not , ty }) = field . ty . kind { return Some (lt . ident . span . between (ty . span)) ; } None }
    };
}

get_mut_span_in_struct_field!();