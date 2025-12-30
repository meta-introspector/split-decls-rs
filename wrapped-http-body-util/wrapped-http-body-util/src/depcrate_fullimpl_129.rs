// Generated macro for impl_129 (impl)
macro_rules! Depcrate_fullimpl_129 {
() => {
// Module: crate::full
// Provides: {"impl_129"}
// Dependencies: {}
impl < D > Full < D > where D : Buf , { # [doc = " Create a new `Full`."] pub fn new (data : D) -> Self { let data = if data . has_remaining () { Some (data) } else { None } ; Full { data } } }
};
}
