macro_rules! deps {
    () => {
        PanicNonStrErr!();
        ConstCx!();
        NonConstOp!();
        PanicNonStr!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for PanicNonStr { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: PanicNonStrErr { span }) } }
    };
}

impl_40!()