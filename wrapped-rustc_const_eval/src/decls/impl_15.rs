macro_rules! deps {
    () => {
        NonConstOp!();
        ConstCx!();
        UnallowedFnPointerCall!();
        FnCallIndirect!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for FnCallIndirect { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: UnallowedFnPointerCall { span , kind : ccx . const_kind () }) } }
    };
}

impl_15!();