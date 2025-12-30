// Generated macro for impl_44 (impl)
macro_rules! Depcrate_csbiimpl_44 {
() => {
// Module: crate::csbi
// Provides: {"impl_44"}
// Dependencies: {}
impl ScreenBufferInfo { # [doc = " Create a new console screen buffer without all zeroed properties."] pub fn new () -> ScreenBufferInfo { ScreenBufferInfo (unsafe { zeroed () }) } # [doc = " Get the size of the screen buffer."] # [doc = ""] # [doc = " Will take `dwSize` from the current screen buffer and convert it into a [`Size`]."] pub fn buffer_size (& self) -> Size { Size :: from (self . 0 . dwSize) } # [doc = " Get the size of the terminal display window."] # [doc = ""] # [doc = " Will calculate the width and height from `srWindow` and convert it into a [`Size`]."] pub fn terminal_size (& self) -> Size { Size :: new (self . 0 . srWindow . Right - self . 0 . srWindow . Left , self . 0 . srWindow . Bottom - self . 0 . srWindow . Top ,) } # [doc = " Get the position and size of the terminal display window."] # [doc = ""] # [doc = " Will take `srWindow` and convert it into the `WindowPositions` type."] pub fn terminal_window (& self) -> WindowPositions { WindowPositions :: from (self . 0) } # [doc = " Get the current attributes of the characters that are being written to the console."] # [doc = ""] # [doc = " Will take `wAttributes` from the current screen buffer."] pub fn attributes (& self) -> u16 { self . 0 . wAttributes } # [doc = " Get the current column and row of the terminal cursor in the screen buffer."] # [doc = ""] # [doc = " Will take `dwCursorPosition` from the current screen buffer."] pub fn cursor_pos (& self) -> Coord { Coord :: from (self . 0 . dwCursorPosition) } }
};
}
