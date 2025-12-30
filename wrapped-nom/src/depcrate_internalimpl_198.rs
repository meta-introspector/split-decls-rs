// Generated macro for impl_198 (impl)
macro_rules! Depcrate_internalimpl_198 {
() => {
// Module: crate::internal
// Provides: {"impl_198"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Err < error :: Error < & str > > { # [doc = " Obtaining ownership"] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn to_owned (self) -> Err < error :: Error < String > > { self . map_input (ToOwned :: to_owned) } }
};
}
