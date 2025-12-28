macro_rules! deps {
    () => {
        ConstCx!();
        Status!();
        NonConstOp!();
        ConditionallyConstCall!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for ConditionallyConstCall < 'tcx > { fn status_in_item (& self , _ccx : & ConstCx < '_ , 'tcx >) -> Status { Status :: Unstable { gate : sym :: const_trait_impl , gate_already_checked : false , safe_to_expose_on_stable : false , is_function_call : false , } } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , _ : Span) -> Diag < 'tcx > { let mut diag = build_error_for_const_call (ccx , self . callee , self . args , self . span , self . call_source , "conditionally" , | _ , _ , _ | { } ,) ; diag . code (E0658) ; add_feature_diagnostics (& mut diag , ccx . tcx . sess , sym :: const_trait_impl) ; diag } }
    };
}

impl_17!();