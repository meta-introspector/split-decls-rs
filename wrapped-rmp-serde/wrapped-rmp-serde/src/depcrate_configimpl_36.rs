// Generated macro for impl_36 (impl)
macro_rules! Depcrate_configimpl_36 {
() => {
// Module: crate::config
// Provides: {"impl_36"}
// Dependencies: {}
impl < C > sealed :: SerializerConfig for StructTupleConfig < C > where C : sealed :: SerializerConfig , { # [inline (always)] fn is_named (& self) -> bool { false } # [inline (always)] fn is_human_readable (& self) -> bool { self . 0 . is_human_readable () } fn bytes (& self) -> BytesMode { self . 0 . bytes () } }
};
}
