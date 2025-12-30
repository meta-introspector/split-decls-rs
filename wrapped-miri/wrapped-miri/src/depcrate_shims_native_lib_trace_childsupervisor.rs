// Generated macro for SUPERVISOR (static)
macro_rules! Depcrate_shims_native_lib_trace_childSUPERVISOR {
() => {
// Module: crate::shims::native_lib::trace::child
// Provides: {"SUPERVISOR"}
// Dependencies: {}
# [doc = " A handle to the single, shared supervisor process across all `MiriMachine`s."] # [doc = " Since it would be very difficult to trace multiple FFI calls in parallel, we"] # [doc = " need to ensure that either (a) only one `MiriMachine` is performing an FFI call"] # [doc = " at any given time, or (b) there are distinct supervisor and child processes for"] # [doc = " each machine. The former was chosen here."] # [doc = ""] # [doc = " This should only contain a `None` if the supervisor has not (yet) been initialised;"] # [doc = " otherwise, if `init_sv` was called and did not error, this will always be nonempty."] static SUPERVISOR : std :: sync :: Mutex < Option < Supervisor > > = std :: sync :: Mutex :: new (None) ;
};
}
