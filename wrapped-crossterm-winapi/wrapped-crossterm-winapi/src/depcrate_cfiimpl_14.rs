// Generated macro for impl_14 (impl)
macro_rules! Depcrate_cfiimpl_14 {
() => {
// Module: crate::cfi
// Provides: {"impl_14"}
// Dependencies: {}
impl FontInfo { # [doc = " Create a new font info without all zeroed properties."] pub fn new () -> FontInfo { FontInfo (unsafe { zeroed () }) } # [doc = " Get the size of the font."] # [doc = ""] # [doc = " Will take `dwFontSize` from the current font info and convert it into a [`Size`]."] pub fn size (& self) -> Size { Size :: from (self . 0 . dwFontSize) } # [doc = " Get the index of the font in the system's console font table."] pub fn index (& self) -> u32 { self . 0 . nFont } }
};
}
