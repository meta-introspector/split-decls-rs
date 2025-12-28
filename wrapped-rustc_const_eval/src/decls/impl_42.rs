macro_rules! deps {
    () => {
        RawPtrComparison!();
        RawPtrComparisonErr!();
        NonConstOp!();
        ConstCx!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for RawPtrComparison { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: RawPtrComparisonErr { span }) } }
    };
}

impl_42!();