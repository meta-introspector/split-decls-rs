// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl IntoCrossterm < CrosstermColor > for Color { fn into_crossterm (self) -> CrosstermColor { match self { Self :: Reset => CrosstermColor :: Reset , Self :: Black => CrosstermColor :: Black , Self :: Red => CrosstermColor :: DarkRed , Self :: Green => CrosstermColor :: DarkGreen , Self :: Yellow => CrosstermColor :: DarkYellow , Self :: Blue => CrosstermColor :: DarkBlue , Self :: Magenta => CrosstermColor :: DarkMagenta , Self :: Cyan => CrosstermColor :: DarkCyan , Self :: Gray => CrosstermColor :: Grey , Self :: DarkGray => CrosstermColor :: DarkGrey , Self :: LightRed => CrosstermColor :: Red , Self :: LightGreen => CrosstermColor :: Green , Self :: LightBlue => CrosstermColor :: Blue , Self :: LightYellow => CrosstermColor :: Yellow , Self :: LightMagenta => CrosstermColor :: Magenta , Self :: LightCyan => CrosstermColor :: Cyan , Self :: White => CrosstermColor :: White , Self :: Indexed (i) => CrosstermColor :: AnsiValue (i) , Self :: Rgb (r , g , b) => CrosstermColor :: Rgb { r , g , b } , } } }
};
}
