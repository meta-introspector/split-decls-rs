// Generated macro for impl_33 (impl)
macro_rules! Depcrate_configimpl_33 {
() => {
// Module: crate::config
// Provides: {"impl_33"}
// Dependencies: {}
impl < C > sealed :: SerializerConfig for StructMapConfig < C > where C : sealed :: SerializerConfig , { # [inline (always)] fn is_named (& self) -> bool { true } # [inline (always)] fn is_human_readable (& self) -> bool { self . 0 . is_human_readable () } fn bytes (& self) -> BytesMode { self . 0 . bytes () } }
};
}
