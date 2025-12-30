// Generated macro for impl_129 (impl)
macro_rules! Depcrate_sync_parkerimpl_129 {
() => {
// Module: crate::sync::parker
// Provides: {"impl_129"}
// Dependencies: {}
impl Default for Parker { fn default () -> Self { Self { unparker : Unparker { inner : Arc :: new (Inner { state : AtomicUsize :: new (EMPTY) , lock : Mutex :: new (()) , cvar : Condvar :: new () , }) , } , _marker : PhantomData , } } }
};
}
