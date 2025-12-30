// Generated macro for impl_180 (impl)
macro_rules! Depcrate_validimpl_180 {
() => {
// Module: crate::valid
// Provides: {"impl_180"}
// Dependencies: {}
impl Input < '_ > { pub (crate) fn validate (& self) -> Result < () > { match self { Input :: Struct (input) => input . validate () , Input :: Enum (input) => input . validate () , } } }
};
}
