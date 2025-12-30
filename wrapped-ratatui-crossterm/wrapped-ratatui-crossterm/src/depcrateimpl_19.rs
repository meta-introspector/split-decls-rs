// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl FromCrossterm < CrosstermColor > for Color { fn from_crossterm (value : CrosstermColor) -> Self { match value { CrosstermColor :: Reset => Self :: Reset , CrosstermColor :: Black => Self :: Black , CrosstermColor :: DarkRed => Self :: Red , CrosstermColor :: DarkGreen => Self :: Green , CrosstermColor :: DarkYellow => Self :: Yellow , CrosstermColor :: DarkBlue => Self :: Blue , CrosstermColor :: DarkMagenta => Self :: Magenta , CrosstermColor :: DarkCyan => Self :: Cyan , CrosstermColor :: Grey => Self :: Gray , CrosstermColor :: DarkGrey => Self :: DarkGray , CrosstermColor :: Red => Self :: LightRed , CrosstermColor :: Green => Self :: LightGreen , CrosstermColor :: Blue => Self :: LightBlue , CrosstermColor :: Yellow => Self :: LightYellow , CrosstermColor :: Magenta => Self :: LightMagenta , CrosstermColor :: Cyan => Self :: LightCyan , CrosstermColor :: White => Self :: White , CrosstermColor :: Rgb { r , g , b } => Self :: Rgb (r , g , b) , CrosstermColor :: AnsiValue (v) => Self :: Indexed (v) , } } }
};
}
