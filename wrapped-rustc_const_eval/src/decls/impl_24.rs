macro_rules! deps {
    () => {
        NonConstIntrinsic!();
        ConstCx!();
        NonConstOp!();
        IntrinsicNonConst!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'tcx > NonConstOp < 'tcx > for IntrinsicNonConst { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: NonConstIntrinsic { span , name : self . name , kind : ccx . const_kind () , }) } }
    };
}

impl_24!()