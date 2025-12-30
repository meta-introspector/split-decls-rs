// Generated macro for impl_162 (impl)
macro_rules! Depcrate_ringimpl_162 {
() => {
// Module: crate::ring
// Provides: {"impl_162"}
// Dependencies: {}
impl < T > Index < usize > for RingBuffer < T > { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { & self . data [index . checked_sub (self . offset) . unwrap ()] } }
};
}
