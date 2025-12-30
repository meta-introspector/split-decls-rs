// Generated macro for impl_39 (impl)
macro_rules! Depcrate_configimpl_39 {
() => {
// Module: crate::config
// Provides: {"impl_39"}
// Dependencies: {}
impl < C > sealed :: SerializerConfig for HumanReadableConfig < C > where C : sealed :: SerializerConfig , { # [inline (always)] fn is_named (& self) -> bool { self . 0 . is_named () } # [inline (always)] fn is_human_readable (& self) -> bool { true } fn bytes (& self) -> BytesMode { self . 0 . bytes () } }
};
}
