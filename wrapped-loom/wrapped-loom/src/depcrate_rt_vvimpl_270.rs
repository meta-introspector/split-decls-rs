// Generated macro for impl_270 (impl)
macro_rules! Depcrate_rt_vvimpl_270 {
() => {
// Module: crate::rt::vv
// Provides: {"impl_270"}
// Dependencies: {}
impl VersionVec { pub (crate) fn new () -> VersionVec { VersionVec { versions : [0 ; MAX_THREADS] , } } pub (crate) fn versions (& self , execution_id : execution :: Id ,) -> impl Iterator < Item = (thread :: Id , u16) > + '_ { self . versions . iter () . enumerate () . map (move | (thread_id , & version) | (thread :: Id :: new (execution_id , thread_id) , version)) } pub (crate) fn inc (& mut self , id : thread :: Id) { self . versions [id . as_usize ()] += 1 ; } pub (crate) fn join (& mut self , other : & VersionVec) { for (i , & version) in other . versions . iter () . enumerate () { self . versions [i] = cmp :: max (self . versions [i] , version) ; } } # [doc = " Returns the thread ID, if any, that is ahead of the current version."] pub (crate) fn ahead (& self , other : & VersionVec) -> Option < usize > { for (i , & version) in other . versions . iter () . enumerate () { if self . versions [i] < version { return Some (i) ; } } None } }
};
}
