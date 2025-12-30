// Generated macro for impl_196 (impl)
macro_rules! Depcrate_internalimpl_196 {
() => {
// Module: crate::internal
// Provides: {"impl_196"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Err < (& str , ErrorKind) > { # [doc = " Obtaining ownership"] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn to_owned (self) -> Err < (String , ErrorKind) > { self . map_input (ToOwned :: to_owned) } }
};
}
