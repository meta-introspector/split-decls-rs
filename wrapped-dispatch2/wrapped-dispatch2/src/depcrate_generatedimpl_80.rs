// Generated macro for impl_80 (impl)
macro_rules! Depcrate_generatedimpl_80 {
() => {
// Module: crate::generated
// Provides: {"impl_80"}
// Dependencies: {}
impl DispatchGroup { # [doc = " Manually indicate a block in the group has completed"] # [doc = ""] # [doc = ""] # [doc = " Calling this function indicates block has completed and left the dispatch"] # [doc = " group by a means other than dispatch_group_async()."] # [doc = ""] # [doc = ""] # [doc = " Parameter `group`: The dispatch group to update."] # [doc = " The result of passing NULL in this parameter is undefined."] # [doc (alias = "dispatch_group_leave")] # [inline] pub unsafe fn leave (& self) { extern "C" { fn dispatch_group_leave (group : & DispatchGroup) ; } unsafe { dispatch_group_leave (self) } } }
};
}
