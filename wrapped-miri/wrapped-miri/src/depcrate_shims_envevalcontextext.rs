// Generated macro for EvalContextExt (trait)
macro_rules! Depcrate_shims_envEvalContextExt {
() => {
// Module: crate::shims::env
// Provides: {"EvalContextExt"}
// Dependencies: {}
pub trait EvalContextExt < 'tcx > : crate :: MiriInterpCxExt < 'tcx > { # [doc = " Try to get an environment variable from the interpreted program's environment. This is"] # [doc = " useful for implementing shims which are documented to read from the environment."] fn get_env_var (& mut self , name : & OsStr) -> InterpResult < 'tcx , Option < OsString > > { let this = self . eval_context_ref () ; match & this . machine . env_vars { EnvVars :: Uninit => interp_ok (None) , EnvVars :: Unix (vars) => vars . get (this , name) , EnvVars :: Windows (vars) => vars . get (name) , } } # [doc = " Get the process identifier."] fn get_pid (& self) -> u32 { let this = self . eval_context_ref () ; if this . machine . communicate () { std :: process :: id () } else { 1000 } } # [doc = " Get an \"OS\" thread ID for the current thread."] fn get_current_tid (& self) -> u32 { let this = self . eval_context_ref () ; self . get_tid (this . machine . threads . active_thread ()) } # [doc = " Get an \"OS\" thread ID for any thread."] fn get_tid (& self , thread : ThreadId) -> u32 { let this = self . eval_context_ref () ; let index = thread . to_u32 () ; let target_os = & this . tcx . sess . target . os ; if target_os == "linux" || target_os == "netbsd" { this . get_pid () . strict_add (index) } else { index } } }
};
}
