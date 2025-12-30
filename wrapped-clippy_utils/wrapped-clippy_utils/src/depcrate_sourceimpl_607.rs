// Generated macro for impl_607 (impl)
macro_rules! Depcrate_sourceimpl_607 {
() => {
// Module: crate::source
// Provides: {"impl_607"}
// Dependencies: {}
impl < T > Index < T > for SourceText where str : Index < T > , { type Output = < str as Index < T > > :: Output ; fn index (& self , idx : T) -> & Self :: Output { & self . as_str () [idx] } }
};
}
