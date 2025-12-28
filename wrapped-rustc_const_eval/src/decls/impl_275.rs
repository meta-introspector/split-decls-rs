macro_rules! deps {
    () => {
        Operand!();
        Immediate!();
        ImmTy!();
        OpTy!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > From < ImmTy < 'tcx , Prov > > for OpTy < 'tcx , Prov > { # [inline (always)] fn from (val : ImmTy < 'tcx , Prov >) -> Self { OpTy { op : Operand :: Immediate (val . imm) , layout : val . layout } } }
    };
}

impl_275!();