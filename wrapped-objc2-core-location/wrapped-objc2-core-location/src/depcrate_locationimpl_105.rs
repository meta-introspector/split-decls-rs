// Generated macro for impl_105 (impl)
macro_rules! Depcrate_locationimpl_105 {
() => {
// Module: crate::location
// Provides: {"impl_105"}
// Dependencies: {}
impl CLLocationCoordinate2D { # [doc (alias = "CLLocationCoordinate2DIsValid")] # [inline] pub unsafe fn is_valid (self) -> bool { extern "C-unwind" { fn CLLocationCoordinate2DIsValid (coord : CLLocationCoordinate2D) -> Bool ; } unsafe { CLLocationCoordinate2DIsValid (self) } . as_bool () } # [doc (alias = "CLLocationCoordinate2DMake")] # [inline] pub unsafe fn new (latitude : CLLocationDegrees , longitude : CLLocationDegrees ,) -> CLLocationCoordinate2D { extern "C-unwind" { fn CLLocationCoordinate2DMake (latitude : CLLocationDegrees , longitude : CLLocationDegrees ,) -> CLLocationCoordinate2D ; } unsafe { CLLocationCoordinate2DMake (latitude , longitude) } } }
};
}
