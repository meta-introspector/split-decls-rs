// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl FromTermwiz < AnsiColor > for Color { fn from_termwiz (value : AnsiColor) -> Self { match value { AnsiColor :: Black => Self :: Black , AnsiColor :: Grey => Self :: DarkGray , AnsiColor :: Silver => Self :: Gray , AnsiColor :: Maroon => Self :: Red , AnsiColor :: Red => Self :: LightRed , AnsiColor :: Green => Self :: Green , AnsiColor :: Lime => Self :: LightGreen , AnsiColor :: Olive => Self :: Yellow , AnsiColor :: Yellow => Self :: LightYellow , AnsiColor :: Purple => Self :: Magenta , AnsiColor :: Fuchsia => Self :: LightMagenta , AnsiColor :: Teal => Self :: Cyan , AnsiColor :: Aqua => Self :: LightCyan , AnsiColor :: White => Self :: White , AnsiColor :: Navy => Self :: Blue , AnsiColor :: Blue => Self :: LightBlue , } } }
};
}
