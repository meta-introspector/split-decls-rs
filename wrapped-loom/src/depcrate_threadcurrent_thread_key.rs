// Generated macro for CURRENT_THREAD_KEY (static)
macro_rules! Depcrate_threadCURRENT_THREAD_KEY {
() => {
// Module: crate::thread
// Provides: {"CURRENT_THREAD_KEY"}
// Dependencies: {}
static CURRENT_THREAD_KEY : LocalKey < Thread > = LocalKey { init : | | unreachable ! () , _p : PhantomData , } ;
};
}
