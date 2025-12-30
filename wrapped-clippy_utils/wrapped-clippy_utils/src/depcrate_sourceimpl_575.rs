// Generated macro for impl_575 (impl)
macro_rules! Depcrate_sourceimpl_575 {
() => {
// Module: crate::source
// Provides: {"impl_575"}
// Dependencies: {}
impl < T > Index < T > for SourceText where str : Index < T > , { type Output = < str as Index < T > > :: Output ; fn index (& self , idx : T) -> & Self :: Output { & self . as_str () [idx] } }
};
}
