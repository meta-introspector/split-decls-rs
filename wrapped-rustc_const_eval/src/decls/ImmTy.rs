macro_rules! deps {
    () => {
        Immediate!();
    };
}

macro_rules! ImmTy {
    () => {
        deps!();
        # [derive (Clone)] pub struct ImmTy < 'tcx , Prov : Provenance = CtfeProvenance > { imm : Immediate < Prov > , pub layout : TyAndLayout < 'tcx > , }
    };
}

ImmTy!();