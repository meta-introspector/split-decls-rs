// Generated macro for impl_59 (impl)
macro_rules! Depcrate_wakerimpl_59 {
() => {
// Module: crate::waker
// Provides: {"impl_59"}
// Dependencies: {}
impl Waker { # [doc = " Create a new `Waker`."] pub fn new (registry : & Registry , token : Token) -> io :: Result < Waker > { # [cfg (debug_assertions)] registry . register_waker () ; sys :: Waker :: new (registry . selector () , token) . map (| inner | Waker { inner }) } # [doc = " Wake up the [`Poll`] associated with this `Waker`."] # [doc = ""] # [doc = " [`Poll`]: struct.Poll.html"] pub fn wake (& self) -> io :: Result < () > { self . inner . wake () } }
};
}
