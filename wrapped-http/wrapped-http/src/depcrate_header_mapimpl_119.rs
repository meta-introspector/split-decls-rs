// Generated macro for impl_119 (impl)
macro_rules! Depcrate_header_mapimpl_119 {
() => {
// Module: crate::header::map
// Provides: {"impl_119"}
// Dependencies: {}
impl < T > ops :: Index < usize > for RawLinks < T > { type Output = Option < Links > ; fn index (& self , idx : usize) -> & Self :: Output { unsafe { & (* self . 0) [idx] . links } } }
};
}
