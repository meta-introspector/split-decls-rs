// Generated macro for impl_13 (impl)
macro_rules! Depcrate_commonimpl_13 {
() => {
// Module: crate::common
// Provides: {"impl_13"}
// Dependencies: {}
impl SectionKind { # [doc = " Return true if this section contains zerofill data."] pub fn is_bss (self) -> bool { self == SectionKind :: UninitializedData || self == SectionKind :: UninitializedTls || self == SectionKind :: Common } }
};
}
