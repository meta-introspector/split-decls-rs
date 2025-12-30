// Generated macro for impl_272 (impl)
macro_rules! Depcrate_rt_vvimpl_272 {
() => {
// Module: crate::rt::vv
// Provides: {"impl_272"}
// Dependencies: {}
impl ops :: Index < thread :: Id > for VersionVec { type Output = u16 ; fn index (& self , index : thread :: Id) -> & u16 { self . versions . index (index . as_usize ()) } }
};
}
