// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < T > ops :: IndexMut < Index < T > > for VecList < T > { fn index_mut (& mut self , index : Index < T >) -> & mut Self :: Output { self . get_mut (index) . expect ("expected entry at index") } }
};
}
