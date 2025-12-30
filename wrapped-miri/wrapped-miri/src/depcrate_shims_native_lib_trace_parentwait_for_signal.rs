// Generated macro for wait_for_signal (function)
macro_rules! Depcrate_shims_native_lib_trace_parentwait_for_signal {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"wait_for_signal"}
// Dependencies: {}
# [doc = " Waits for `wait_signal`. If `init_cont`, it will first do a `ptrace::cont`."] # [doc = " We want to avoid that in some cases, like at the beginning of FFI."] # [doc = ""] # [doc = " If `pid` is `None`, only one wait will be done and `init_cont` should be false."] fn wait_for_signal (pid : Option < unistd :: Pid > , wait_signal : signal :: Signal , init_cont : InitialCont ,) -> Result < unistd :: Pid , ExecEnd > { if matches ! (init_cont , InitialCont :: Yes) { ptrace :: cont (pid . unwrap () , None) . unwrap () ; } loop { let wait_id = match pid { Some (pid) => wait :: Id :: Pid (pid) , None => wait :: Id :: All , } ; let stat = wait :: waitid (wait_id , WAIT_FLAGS) . map_err (| _ | ExecEnd (None)) ? ; let (signal , pid) = match stat { wait :: WaitStatus :: Exited (_ , code) => { return Err (ExecEnd (Some (code))) ; } wait :: WaitStatus :: Signaled (_ , _ , _) => return Err (ExecEnd (None)) , wait :: WaitStatus :: Stopped (pid , signal) | wait :: WaitStatus :: PtraceEvent (pid , signal , _) => (signal , pid) , _ => { ptrace :: cont (pid . unwrap () , None) . unwrap () ; continue ; } } ; if signal == wait_signal { return Ok (pid) ; } else { ptrace :: cont (pid , signal) . map_err (| _ | ExecEnd (None)) ? ; } } }
};
}
