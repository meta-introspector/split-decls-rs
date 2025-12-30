// Generated macro for UnparkMutex (struct)
macro_rules! Depcrate_unpark_mutexUnparkMutex {
() => {
// Module: crate::unpark_mutex
// Provides: {"UnparkMutex"}
// Dependencies: {}
# [doc = " A \"lock\" around data `D`, which employs a *helping* strategy."] # [doc = ""] # [doc = " Used to ensure that concurrent `unpark` invocations lead to (1) `poll` being"] # [doc = " invoked on only a single thread at a time (2) `poll` being invoked at least"] # [doc = " once after each `unpark` (unless the future has completed)."] pub (crate) struct UnparkMutex < D > { status : AtomicUsize , inner : UnsafeCell < Option < D > > , }
};
}
