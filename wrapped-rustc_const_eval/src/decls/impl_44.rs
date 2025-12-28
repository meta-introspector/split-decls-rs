macro_rules! deps {
    () => {
        RawPtrToIntCast!();
        NonConstOp!();
        ConstCx!();
        RawPtrToIntErr!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for RawPtrToIntCast { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: RawPtrToIntErr { span }) } }
    };
}

impl_44!()