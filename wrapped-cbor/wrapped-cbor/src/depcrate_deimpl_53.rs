// Generated macro for impl_53 (impl)
macro_rules! Depcrate_deimpl_53 {
() => {
// Module: crate::de
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'de , R , T > StreamDeserializer < 'de , R , T > where R : Offset , T : de :: Deserialize < 'de > , { # [doc = " Return the current offset in the reader"] # [inline] pub fn byte_offset (& self) -> usize { self . de . byte_offset () } }
};
}
