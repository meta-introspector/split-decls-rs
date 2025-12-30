// Generated macro for impl_42 (impl)
macro_rules! Depcrate_configimpl_42 {
() => {
// Module: crate::config
// Provides: {"impl_42"}
// Dependencies: {}
impl < C > sealed :: SerializerConfig for BinaryConfig < C > where C : sealed :: SerializerConfig , { # [inline (always)] fn is_named (& self) -> bool { self . 0 . is_named () } # [inline (always)] fn is_human_readable (& self) -> bool { false } fn bytes (& self) -> BytesMode { self . 0 . bytes () } }
};
}
