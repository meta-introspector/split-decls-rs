// Generated macro for impl_630 (impl)
macro_rules! Depcrate_interpret_placeimpl_630 {
() => {
// Module: crate::interpret::place
// Provides: {"impl_630"}
// Dependencies: {}
impl < 'tcx , Prov : Provenance > Writeable < 'tcx , Prov > for MPlaceTy < 'tcx , Prov > { # [inline (always)] fn to_place (& self) -> PlaceTy < 'tcx , Prov > { self . clone () . into () } # [inline (always)] fn force_mplace < M : Machine < 'tcx , Provenance = Prov > > (& self , _ecx : & mut InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , MPlaceTy < 'tcx , Prov > > { interp_ok (self . clone ()) } }
};
}
