// Generated macro for impl_163 (impl)
macro_rules! Depcrate_ringimpl_163 {
() => {
// Module: crate::ring
// Provides: {"impl_163"}
// Dependencies: {}
impl < T > IndexMut < usize > for RingBuffer < T > { fn index_mut (& mut self , index : usize) -> & mut Self :: Output { & mut self . data [index . checked_sub (self . offset) . unwrap ()] } }
};
}
