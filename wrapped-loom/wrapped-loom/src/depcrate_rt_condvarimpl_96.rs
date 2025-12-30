// Generated macro for impl_96 (impl)
macro_rules! Depcrate_rt_condvarimpl_96 {
() => {
// Module: crate::rt::condvar
// Provides: {"impl_96"}
// Dependencies: {}
impl State { pub (super) fn last_dependent_access (& self) -> Option < & Access > { self . last_access . as_ref () } pub (crate) fn set_last_access (& mut self , path_id : usize , version : & VersionVec) { Access :: set_or_create (& mut self . last_access , path_id , version) ; } }
};
}
