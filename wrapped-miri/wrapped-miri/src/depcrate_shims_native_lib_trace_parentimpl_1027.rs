// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_shims_native_lib_trace_parentimpl_1027 {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"impl_1027"}
// Dependencies: {}
impl Iterator for ChildListener { type Item = ExecEvent ; fn next (& mut self) -> Option < Self :: Item > { let opts = WAIT_FLAGS | wait :: WaitPidFlag :: WNOHANG ; loop { match wait :: waitid (wait :: Id :: All , opts) { Ok (stat) => match stat { wait :: WaitStatus :: Exited (_ , code) => self . last_code = Some (code) , wait :: WaitStatus :: Signaled (_ , _ , _) => self . last_code = None , wait :: WaitStatus :: PtraceSyscall (pid) => if self . attached { return Some (ExecEvent :: Syscall (pid)) ; } , wait :: WaitStatus :: Stopped (pid , signal) | wait :: WaitStatus :: PtraceEvent (pid , signal , _) => if self . attached { if signal == signal :: SIGUSR1 { self . attached = false ; return Some (ExecEvent :: End) ; } else { return Some (ExecEvent :: Status (pid , signal)) ; } } else { ptrace :: cont (pid , signal) . unwrap () ; } , _ => () , } , Err (_) => return Some (ExecEvent :: Died (self . override_retcode . or (self . last_code))) , } if let Ok (req) = self . message_rx . try_recv () { match req { TraceRequest :: StartFfi (info) => if self . attached { panic ! ("Attempting to begin FFI multiple times!") ; } else { self . attached = true ; return Some (ExecEvent :: Start (info)) ; } , TraceRequest :: OverrideRetcode (code) => { self . override_retcode = Some (code) ; self . confirm_tx . send (Confirmation) . unwrap () ; } } } std :: thread :: yield_now () ; } } }
};
}
