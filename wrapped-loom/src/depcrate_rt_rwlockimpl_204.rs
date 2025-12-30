// Generated macro for impl_204 (impl)
macro_rules! Depcrate_rt_rwlockimpl_204 {
() => {
// Module: crate::rt::rwlock
// Provides: {"impl_204"}
// Dependencies: {}
impl State { pub (crate) fn last_dependent_access (& self) -> Option < & Access > { self . last_access . as_ref () } pub (crate) fn set_last_access (& mut self , path_id : usize , version : & VersionVec) { Access :: set_or_create (& mut self . last_access , path_id , version) } }
};
}
