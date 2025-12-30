// Generated macro for macro_39 (macro)
macro_rules! Depcrate_threadingmacro_39 {
() => {
// Module: crate::threading
// Provides: {"macro_39"}
// Dependencies: {}
thread_local ! { static THREAD_SHUTDOWN : OnceCell < (Sender < () >, Receiver < () >) > = const { OnceCell :: new () } ; }
};
}
