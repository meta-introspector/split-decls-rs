// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl IntoTermwiz < ColorAttribute > for Color { fn into_termwiz (self) -> ColorAttribute { match self { Self :: Reset => ColorAttribute :: Default , Self :: Black => AnsiColor :: Black . into () , Self :: DarkGray => AnsiColor :: Grey . into () , Self :: Gray => AnsiColor :: Silver . into () , Self :: Red => AnsiColor :: Maroon . into () , Self :: LightRed => AnsiColor :: Red . into () , Self :: Green => AnsiColor :: Green . into () , Self :: LightGreen => AnsiColor :: Lime . into () , Self :: Yellow => AnsiColor :: Olive . into () , Self :: LightYellow => AnsiColor :: Yellow . into () , Self :: Magenta => AnsiColor :: Purple . into () , Self :: LightMagenta => AnsiColor :: Fuchsia . into () , Self :: Cyan => AnsiColor :: Teal . into () , Self :: LightCyan => AnsiColor :: Aqua . into () , Self :: White => AnsiColor :: White . into () , Self :: Blue => AnsiColor :: Navy . into () , Self :: LightBlue => AnsiColor :: Blue . into () , Self :: Indexed (i) => ColorAttribute :: PaletteIndex (i) , Self :: Rgb (r , g , b) => { ColorAttribute :: TrueColorWithDefaultFallback (SrgbaTuple :: from ((r , g , b))) } } } }
};
}
