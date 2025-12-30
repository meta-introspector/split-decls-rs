// Generated macro for MPlaceTy (struct)
macro_rules! Depcrate_interpret_placeMPlaceTy {
() => {
// Module: crate::interpret::place
// Provides: {"MPlaceTy"}
// Dependencies: {}
# [doc = " A MemPlace with its layout. Constructing it is only possible in this module."] # [derive (Clone , Hash , Eq , PartialEq)] pub struct MPlaceTy < 'tcx , Prov : Provenance = CtfeProvenance > { mplace : MemPlace < Prov > , pub layout : TyAndLayout < 'tcx > , }
};
}
