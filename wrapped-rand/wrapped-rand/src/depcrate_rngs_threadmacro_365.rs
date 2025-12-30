// Generated macro for macro_365 (macro)
macro_rules! Depcrate_rngs_threadmacro_365 {
() => {
// Module: crate::rngs::thread
// Provides: {"macro_365"}
// Dependencies: {}
thread_local ! (static THREAD_RNG_KEY : Rc < UnsafeCell < ReseedingRng < Core , OsRng >>> = { let rng = ReseedingRng :: new (THREAD_RNG_RESEED_THRESHOLD , OsRng) . unwrap_or_else (| err | panic ! ("could not initialize ThreadRng: {}" , err)) ; Rc :: new (UnsafeCell :: new (rng)) }) ;
};
}
