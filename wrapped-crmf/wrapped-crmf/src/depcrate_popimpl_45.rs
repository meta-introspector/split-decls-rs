// Generated macro for impl_45 (impl)
macro_rules! Depcrate_popimpl_45 {
() => {
// Module: crate::pop
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'a > :: der :: Choice < 'a > for EncKeyWithIdChoice < 'a > { fn can_decode (tag : :: der :: Tag) -> bool { < Utf8StringRef < 'a > as :: der :: FixedTag > :: TAG == tag || tag . is_context_specific () } }
};
}
