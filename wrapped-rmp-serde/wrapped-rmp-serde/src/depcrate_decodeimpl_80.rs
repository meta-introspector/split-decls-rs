// Generated macro for impl_80 (impl)
macro_rules! Depcrate_decodeimpl_80 {
() => {
// Module: crate::decode
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'de , R : ReadSlice < 'de > , C : SerializerConfig > Deserializer < R , C > { # [doc = " Changes the maximum nesting depth that is allowed"] # [inline (always)] pub fn set_max_depth (& mut self , depth : usize) { self . depth = depth . min (u16 :: MAX as _) as u16 ; } }
};
}
