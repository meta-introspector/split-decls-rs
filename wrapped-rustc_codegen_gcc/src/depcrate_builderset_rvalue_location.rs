// Generated macro for set_rvalue_location (function)
macro_rules! Depcrate_builderset_rvalue_location {
() => {
// Module: crate::builder
// Provides: {"set_rvalue_location"}
// Dependencies: {}
fn set_rvalue_location < 'a , 'gcc , 'tcx > (bx : & mut Builder < 'a , 'gcc , 'tcx > , rvalue : RValue < 'gcc > ,) -> RValue < 'gcc > { if bx . location . is_some () { # [cfg (feature = "master")] rvalue . set_location (bx . location . unwrap ()) ; } rvalue }
};
}
