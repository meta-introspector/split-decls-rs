// Generated macro for impl_629 (impl)
macro_rules! Depcrate_interpret_placeimpl_629 {
() => {
// Module: crate::interpret::place
// Provides: {"impl_629"}
// Dependencies: {}
impl < 'tcx , Prov : Provenance > Writeable < 'tcx , Prov > for PlaceTy < 'tcx , Prov > { # [inline (always)] fn to_place (& self) -> PlaceTy < 'tcx , Prov > { self . clone () } # [inline (always)] fn force_mplace < M : Machine < 'tcx , Provenance = Prov > > (& self , ecx : & mut InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , MPlaceTy < 'tcx , Prov > > { ecx . force_allocation (self) } }
};
}
