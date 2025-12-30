// Generated macro for impl_275 (impl)
macro_rules! Depcrate_serimpl_275 {
() => {
// Module: crate::ser
// Provides: {"impl_275"}
// Dependencies: {}
# [cfg (feature = "bytes")] impl BorshSerialize for bytes :: BytesMut { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_ref () . serialize (writer) } }
};
}
