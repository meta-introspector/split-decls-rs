// Generated macro for impl_33 (impl)
macro_rules! Depcrate_internimpl_33 {
() => {
// Module: crate::intern
// Provides: {"impl_33"}
// Dependencies: {}
impl < T > Index < Token > for Interner < T > { type Output = T ; fn index (& self , index : Token) -> & Self :: Output { & self . tokens [index . 0 as usize] } }
};
}
