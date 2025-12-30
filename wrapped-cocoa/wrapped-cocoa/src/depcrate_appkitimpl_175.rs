// Generated macro for impl_175 (impl)
macro_rules! Depcrate_appkitimpl_175 {
() => {
// Module: crate::appkit
// Provides: {"impl_175"}
// Dependencies: {}
impl NSEventMask { pub fn from_type (ty : NSEventType) -> NSEventMask { NSEventMask :: from_bits_truncate (1 << ty as c_ulonglong) } }
};
}
