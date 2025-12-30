// Generated macro for impl_66 (impl)
macro_rules! Depcrate_errorimpl_66 {
() => {
// Module: crate::error
// Provides: {"impl_66"}
// Dependencies: {}
impl < E : Display > ErrorExtensions for & E { fn extend (& self) -> Error { Error { message : self . to_string () , source : None , extensions : None , } } }
};
}
