// Generated macro for CLLocationCoordinate2DIsValid (function)
macro_rules! Depcrate_locationCLLocationCoordinate2DIsValid {
() => {
// Module: crate::location
// Provides: {"CLLocationCoordinate2DIsValid"}
// Dependencies: {}
# [deprecated = "renamed to `CLLocationCoordinate2D::is_valid`"] # [inline] pub unsafe extern "C-unwind" fn CLLocationCoordinate2DIsValid (coord : CLLocationCoordinate2D ,) -> bool { extern "C-unwind" { fn CLLocationCoordinate2DIsValid (coord : CLLocationCoordinate2D) -> Bool ; } unsafe { CLLocationCoordinate2DIsValid (coord) } . as_bool () }
};
}
