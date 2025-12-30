// Generated macro for impl_273 (impl)
macro_rules! Depcrate_rt_vvimpl_273 {
() => {
// Module: crate::rt::vv
// Provides: {"impl_273"}
// Dependencies: {}
impl ops :: IndexMut < thread :: Id > for VersionVec { fn index_mut (& mut self , index : thread :: Id) -> & mut u16 { self . versions . index_mut (index . as_usize ()) } }
};
}
