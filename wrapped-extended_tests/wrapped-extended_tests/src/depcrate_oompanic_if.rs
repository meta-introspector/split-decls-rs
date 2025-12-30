// Generated macro for panic_if (function)
macro_rules! Depcrate_oompanic_if {
() => {
// Module: crate::oom
// Provides: {"panic_if"}
// Dependencies: {}
fn panic_if < F : FnOnce () -> bool > (f : F) { if OOM_TEST . load (Relaxed) && ! IN_PANIC . load (Relaxed) { IN_PANIC . swap (true , Relaxed) ; if f () { IN_PANIC . store (true , Relaxed) ; PANIC_COUNT . fetch_add (1 , Relaxed) ; panic ! ("Emulate failure") ; } else { IN_PANIC . store (false , Relaxed) ; } } }
};
}
