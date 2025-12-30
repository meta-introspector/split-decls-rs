// Generated macro for impl_48 (impl)
macro_rules! Depcrate_popimpl_48 {
() => {
// Module: crate::pop
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a > :: der :: Tagged for EncKeyWithIdChoice < 'a > { fn tag (& self) -> :: der :: Tag { match self { Self :: String (_) => < Utf8StringRef < 'a > as :: der :: FixedTag > :: TAG , Self :: GeneralName (variant) => variant . tag () , } } }
};
}
