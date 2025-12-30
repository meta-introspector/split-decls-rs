// Generated macro for impl_195 (impl)
macro_rules! Depcrate_internalimpl_195 {
() => {
// Module: crate::internal
// Provides: {"impl_195"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Err < (& [u8] , ErrorKind) > { # [doc = " Obtaining ownership"] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn to_owned (self) -> Err < (Vec < u8 > , ErrorKind) > { self . map_input (ToOwned :: to_owned) } }
};
}
