// Generated macro for tests (module)
macro_rules! Depcrate_tracingtests {
() => {
// Module: crate::tracing
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: TraceLevel ; # [test] fn smoke () { super :: trace_set (TraceLevel :: Trace , | level , msg | { dbg ! (level , msg) ; }) . expect ("libgit2 can set global trace callback") ; } }
};
}
