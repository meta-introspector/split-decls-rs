macro_rules! deps {
    () => {
        NonConstOp!();
        ConstCx!();
        LiveDrop!();
        Status!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for LiveDrop < 'tcx > { fn status_in_item (& self , _ccx : & ConstCx < '_ , 'tcx >) -> Status { if self . needs_non_const_drop { Status :: Forbidden } else { Status :: Unstable { gate : sym :: const_destruct , gate_already_checked : false , safe_to_expose_on_stable : false , is_function_call : false , } } } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { if self . needs_non_const_drop { ccx . dcx () . create_err (errors :: LiveDrop { span , dropped_ty : self . dropped_ty , kind : ccx . const_kind () , dropped_at : self . dropped_at , }) } else { ccx . tcx . sess . create_feature_err (errors :: LiveDrop { span , dropped_ty : self . dropped_ty , kind : ccx . const_kind () , dropped_at : self . dropped_at , } , sym :: const_destruct ,) } } }
    };
}

impl_34!()