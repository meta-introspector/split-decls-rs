macro_rules! deps {
    () => {
        OpTy!();
        Operand!();
        MPlaceTy!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < 'tcx , Prov : Provenance > From < MPlaceTy < 'tcx , Prov > > for OpTy < 'tcx , Prov > { # [inline (always)] fn from (mplace : MPlaceTy < 'tcx , Prov >) -> Self { OpTy { op : Operand :: Indirect (* mplace . mplace ()) , layout : mplace . layout } } }
    };
}

impl_276!()