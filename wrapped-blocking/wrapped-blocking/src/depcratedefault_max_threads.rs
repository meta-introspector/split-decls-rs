// Generated macro for DEFAULT_MAX_THREADS (const)
macro_rules! DepcrateDEFAULT_MAX_THREADS {
() => {
// Module: crate
// Provides: {"DEFAULT_MAX_THREADS"}
// Dependencies: {}
# [doc = " Default value for max threads that Executor can grow to"] # [cfg (not (target_family = "wasm"))] const DEFAULT_MAX_THREADS : NonZeroUsize = { if let Some (size) = NonZeroUsize :: new (500) { size } else { panic ! ("DEFAULT_MAX_THREADS is non-zero") ; } } ;
};
}
