// Generated macro for impl_13 (impl)
macro_rules! Depcrate_prjimpl_13 {
() => {
// Module: crate::prj
// Provides: {"impl_13"}
// Dependencies: {}
impl From < String > for Channel { fn from (value : String) -> Self { match value . to_lowercase () . as_str () { "stable" => Channel :: Stable , "beta" => Channel :: Beta , "nightly" => Channel :: Nightly , _ => Channel :: Custom (value) , } } }
};
}
