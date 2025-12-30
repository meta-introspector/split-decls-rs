// Generated macro for impl_119 (impl)
macro_rules! Depcrate_rt_notifyimpl_119 {
() => {
// Module: crate::rt::notify
// Provides: {"impl_119"}
// Dependencies: {}
impl State { pub (crate) fn might_spur (& self) -> bool { self . spurious && ! self . did_spur } pub (crate) fn last_dependent_access (& self) -> Option < & Access > { self . last_access . as_ref () } pub (crate) fn set_last_access (& mut self , path_id : usize , version : & VersionVec) { Access :: set_or_create (& mut self . last_access , path_id , version) ; } }
};
}
