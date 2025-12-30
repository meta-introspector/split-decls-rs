// Generated macro for impl_814 (impl)
macro_rules! Depcrate_style_colorimpl_814 {
() => {
// Module: crate::style::color
// Provides: {"impl_814"}
// Dependencies: {}
impl < P : Palette > PaletteColor < P > { # [doc = " Pick a color from the palette"] pub fn pick (idx : usize) -> PaletteColor < P > { PaletteColor (idx % P :: COLORS . len () , PhantomData) } }
};
}
