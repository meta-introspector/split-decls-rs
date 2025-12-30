// Generated macro for Palette (trait)
macro_rules! Depcrate_style_palettePalette {
() => {
// Module: crate::style::palette
// Provides: {"Palette"}
// Dependencies: {}
# [doc = " Represents a color palette"] pub trait Palette { # [doc = " Array of colors"] const COLORS : & 'static [(u8 , u8 , u8)] ; # [doc = " Returns a color from the palette"] fn pick (idx : usize) -> PaletteColor < Self > where Self : Sized , { PaletteColor :: < Self > :: pick (idx) } }
};
}
