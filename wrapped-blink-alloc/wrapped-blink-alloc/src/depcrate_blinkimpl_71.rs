// Generated macro for impl_71 (impl)
macro_rules! Depcrate_blinkimpl_71 {
() => {
// Module: crate::blink
// Provides: {"impl_71"}
// Dependencies: {}
impl < A > SendBlink < A > where A : BlinkAllocator , { # [doc = " Creates new [`SendBlink`] from [`Blink`]."] # [doc = " Resets the blink allocator to avoid dropping non-sendable types on other threads."] # [inline (always)] pub fn new (mut blink : Blink < A >) -> Self { blink . reset () ; SendBlink { blink } } # [doc = " Returns inner [`Blink`] value."] # [inline (always)] pub fn into_inner (self) -> Blink < A > { self . blink } }
};
}
