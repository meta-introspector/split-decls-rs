// Generated macro for macro_55 (macro)
macro_rules! Depcrate_mutexmacro_55 {
() => {
// Module: crate::mutex
// Provides: {"macro_55"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [doc = " Future for acquiring the mutex slowly."] struct AcquireSlow < B : Borrow < Mutex < T >>, T : ? Sized > { mutex : Option < B >, listener : Option < EventListener >, start : Start , starved : bool , # [pin] _marker : PhantomData < T >, # [pin] _pin : PhantomPinned } impl < T : ? Sized , B : Borrow < Mutex < T >>> PinnedDrop for AcquireSlow < B , T > { fn drop (this : Pin <& mut Self >) { this . take_mutex () ; } } }
};
}
