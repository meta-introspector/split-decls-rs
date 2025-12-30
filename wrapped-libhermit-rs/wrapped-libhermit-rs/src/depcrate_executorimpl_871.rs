// Generated macro for impl_871 (impl)
macro_rules! Depcrate_executorimpl_871 {
() => {
// Module: crate::executor
// Provides: {"impl_871"}
// Dependencies: {}
impl WakerRegistration { pub const fn new () -> Self { Self { waker : None } } # [doc = " Register a waker. Overwrites the previous waker, if any."] pub fn register (& mut self , w : & Waker) { match self . waker { Some (ref w2) if (w2 . will_wake (w)) => { } _ => self . waker = Some (w . clone ()) , } } # [doc = " Wake the registered waker, if any."] # [allow (dead_code)] pub fn wake (& mut self) { if let Some (w) = self . waker . take () { w . wake () ; } } }
};
}
