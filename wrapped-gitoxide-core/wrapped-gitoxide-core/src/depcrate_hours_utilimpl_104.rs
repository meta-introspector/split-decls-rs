// Generated macro for impl_104 (impl)
macro_rules! Depcrate_hours_utilimpl_104 {
() => {
// Module: crate::hours::util
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a > From < & 'a WorkByEmail > for WorkByPerson { fn from (w : & 'a WorkByEmail) -> Self { WorkByPerson { name : vec ! [w . name] , email : vec ! [w . email] , hours : w . hours , num_commits : w . num_commits , files : w . files , lines : w . lines , } } }
};
}
