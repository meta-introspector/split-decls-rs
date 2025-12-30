// Generated macro for impl_147 (impl)
macro_rules! Depcrate_workerimpl_147 {
() => {
// Module: crate::worker
// Provides: {"impl_147"}
// Dependencies: {}
impl WorkerCounter { pub (crate) fn new (idx : usize , waker_queue : WakerQueue , counter : Counter) -> Self { Self { idx , inner : Rc :: new ((waker_queue , counter)) , } } # [inline (always)] pub (crate) fn guard (& self) -> WorkerCounterGuard { WorkerCounterGuard (self . clone ()) } fn total (& self) -> usize { self . inner . 1 . total () } }
};
}
