// Generated macro for impl_46 (impl)
macro_rules! Depcrate_errorimpl_46 {
() => {
// Module: crate::error
// Provides: {"impl_46"}
// Dependencies: {}
impl From < parser :: Error > for ServerError { fn from (e : parser :: Error) -> Self { Self { message : e . to_string () , source : None , locations : e . positions () . collect () , path : Vec :: new () , extensions : None , } } }
};
}
