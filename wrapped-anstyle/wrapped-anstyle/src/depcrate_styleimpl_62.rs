// Generated macro for impl_62 (impl)
macro_rules! Depcrate_styleimpl_62 {
() => {
// Module: crate::style
// Provides: {"impl_62"}
// Dependencies: {}
impl core :: fmt :: Display for Style { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { if f . alternate () { self . render_reset () . fmt (f) } else { self . fmt_to (f) } } }
};
}
