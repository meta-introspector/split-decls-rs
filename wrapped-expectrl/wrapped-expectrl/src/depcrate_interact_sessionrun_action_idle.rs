// Generated macro for run_action_idle (function)
macro_rules! Depcrate_interact_sessionrun_action_idle {
() => {
// Module: crate::interact::session
// Provides: {"run_action_idle"}
// Dependencies: {}
# [rustfmt :: skip] fn run_action_idle < S , I , O , C > (s : & mut InteractSession < S , I , O , C > , buf : & [u8] , eof : bool) -> ExpectResult < bool > { let ctx = Context :: new (& mut s . session , & mut s . input , & mut s . output , & mut s . opts . state , buf , eof) ; opt_action (ctx , & mut s . opts . idle_action) }
};
}
