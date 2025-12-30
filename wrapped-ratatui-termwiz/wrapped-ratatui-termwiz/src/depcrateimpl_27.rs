// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl FromTermwiz < ColorAttribute > for Color { fn from_termwiz (value : ColorAttribute) -> Self { match value { ColorAttribute :: TrueColorWithDefaultFallback (srgba) | ColorAttribute :: TrueColorWithPaletteFallback (srgba , _) => srgba . into_ratatui () , ColorAttribute :: PaletteIndex (i) => Self :: Indexed (i) , ColorAttribute :: Default => Self :: Reset , } } }
};
}
