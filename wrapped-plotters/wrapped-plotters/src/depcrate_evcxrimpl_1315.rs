// Generated macro for impl_1315 (impl)
macro_rules! Depcrate_evcxrimpl_1315 {
() => {
// Module: crate::evcxr
// Provides: {"impl_1315"}
// Dependencies: {}
impl SVGWrapper { # [doc = " Displays the contents of the `SVGWrapper` struct."] pub fn evcxr_display (& self) { println ! ("{:?}" , self) ; } # [doc = " Sets the style of the `SVGWrapper` struct."] pub fn style < S : Into < String > > (mut self , style : S) -> Self { self . 1 = style . into () ; self } }
};
}
