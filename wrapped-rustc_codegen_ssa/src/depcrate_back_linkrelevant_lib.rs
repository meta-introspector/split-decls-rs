// Generated macro for relevant_lib (function)
macro_rules! Depcrate_back_linkrelevant_lib {
() => {
// Module: crate::back::link
// Provides: {"relevant_lib"}
// Dependencies: {}
fn relevant_lib (sess : & Session , lib : & NativeLib) -> bool { match lib . cfg { Some (ref cfg) => { eval_config_entry (sess , cfg , CRATE_NODE_ID , None , ShouldEmit :: ErrorsAndLints) . as_bool () } None => true , } }
};
}
