// Generated macro for impl_274 (impl)
macro_rules! Depcrate_serimpl_274 {
() => {
// Module: crate::ser
// Provides: {"impl_274"}
// Dependencies: {}
# [cfg (feature = "bytes")] impl BorshSerialize for bytes :: Bytes { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_ref () . serialize (writer) } }
};
}
