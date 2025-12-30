// Generated macro for impl_624 (impl)
macro_rules! Depcrate_interpret_placeimpl_624 {
() => {
// Module: crate::interpret::place
// Provides: {"impl_624"}
// Dependencies: {}
impl < 'tcx , Prov : Provenance > From < MPlaceTy < 'tcx , Prov > > for PlaceTy < 'tcx , Prov > { # [inline (always)] fn from (mplace : MPlaceTy < 'tcx , Prov >) -> Self { PlaceTy { place : Place :: Ptr (mplace . mplace) , layout : mplace . layout } } }
};
}
