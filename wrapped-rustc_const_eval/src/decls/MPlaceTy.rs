macro_rules! deps {
    () => {
        MemPlace!();
    };
}

macro_rules! MPlaceTy {
    () => {
        deps!();
        # [doc = " A MemPlace with its layout. Constructing it is only possible in this module."] # [derive (Clone , Hash , Eq , PartialEq)] pub struct MPlaceTy < 'tcx , Prov : Provenance = CtfeProvenance > { mplace : MemPlace < Prov > , pub layout : TyAndLayout < 'tcx > , }
    };
}

MPlaceTy!()