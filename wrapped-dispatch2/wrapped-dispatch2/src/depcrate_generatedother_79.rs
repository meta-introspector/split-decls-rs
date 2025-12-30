// Generated macro for other_79 (other)
macro_rules! Depcrate_generatedother_79 {
() => {
// Module: crate::generated
// Provides: {"other_79"}
// Dependencies: {}
extern "C" { # [doc = " Manually indicate a block has entered the group"] # [doc = ""] # [doc = ""] # [doc = " Calling this function indicates another block has joined the group through"] # [doc = " a means other than dispatch_group_async(). Calls to this function must be"] # [doc = " balanced with dispatch_group_leave()."] # [doc = ""] # [doc = ""] # [doc = " Parameter `group`: The dispatch group to update."] # [doc = " The result of passing NULL in this parameter is undefined."] pub fn dispatch_group_enter (group : & DispatchGroup) ; }
};
}
