// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl FromTermwiz < ColorSpec > for Color { fn from_termwiz (value : ColorSpec) -> Self { match value { ColorSpec :: Default => Self :: Reset , ColorSpec :: PaletteIndex (i) => Self :: Indexed (i) , ColorSpec :: TrueColor (srgba) => srgba . into_ratatui () , } } }
};
}
