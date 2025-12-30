// Generated macro for impl_120 (impl)
macro_rules! Depcrate_header_mapimpl_120 {
() => {
// Module: crate::header::map
// Provides: {"impl_120"}
// Dependencies: {}
impl < T > ops :: IndexMut < usize > for RawLinks < T > { fn index_mut (& mut self , idx : usize) -> & mut Self :: Output { unsafe { & mut (* self . 0) [idx] . links } } }
};
}
