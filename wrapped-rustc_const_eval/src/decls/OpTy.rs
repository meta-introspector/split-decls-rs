macro_rules! deps {
    () => {
        Operand!();
    };
}

macro_rules! OpTy {
    () => {
        deps!();
        # [derive (Clone)] pub struct OpTy < 'tcx , Prov : Provenance = CtfeProvenance > { op : Operand < Prov > , pub layout : TyAndLayout < 'tcx > , }
    };
}

OpTy!()