// Generated macro for impl_197 (impl)
macro_rules! Depcrate_internalimpl_197 {
() => {
// Module: crate::internal
// Provides: {"impl_197"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Err < error :: Error < & [u8] > > { # [doc = " Obtaining ownership"] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn to_owned (self) -> Err < error :: Error < Vec < u8 > > > { self . map_input (ToOwned :: to_owned) } }
};
}
