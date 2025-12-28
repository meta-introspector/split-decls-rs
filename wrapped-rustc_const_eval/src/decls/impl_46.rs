macro_rules! deps {
    () => {
        ThreadLocalAccessErr!();
        ConstCx!();
        NonConstOp!();
        ThreadLocalAccess!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for ThreadLocalAccess { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: ThreadLocalAccessErr { span }) } }
    };
}

impl_46!();