macro_rules! deps {
    () => {
        Status!();
        UnallowedOpInConstContext!();
        NonConstOp!();
        ConstCx!();
        Coroutine!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for Coroutine { fn status_in_item (& self , _ : & ConstCx < '_ , 'tcx >) -> Status { match self . 0 { hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: Async , hir :: CoroutineSource :: Block ,) | hir :: CoroutineKind :: Coroutine (_) => Status :: Unstable { gate : sym :: const_async_blocks , gate_already_checked : false , safe_to_expose_on_stable : false , is_function_call : false , } , _ => Status :: Forbidden , } } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { let msg = format ! ("{} are not allowed in {}s" , self . 0 . to_plural_string () , ccx . const_kind ()) ; if let Status :: Unstable { gate , .. } = self . status_in_item (ccx) { ccx . tcx . sess . create_feature_err (errors :: UnallowedOpInConstContext { span , msg } , gate) } else { ccx . dcx () . create_err (errors :: UnallowedOpInConstContext { span , msg }) } } }
    };
}

impl_28!();