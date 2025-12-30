// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for Bg { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { Color :: Reset => termion :: color :: Reset . write_bg (f) , Color :: Black => termion :: color :: Black . write_bg (f) , Color :: Red => termion :: color :: Red . write_bg (f) , Color :: Green => termion :: color :: Green . write_bg (f) , Color :: Yellow => termion :: color :: Yellow . write_bg (f) , Color :: Blue => termion :: color :: Blue . write_bg (f) , Color :: Magenta => termion :: color :: Magenta . write_bg (f) , Color :: Cyan => termion :: color :: Cyan . write_bg (f) , Color :: Gray => termion :: color :: White . write_bg (f) , Color :: DarkGray => termion :: color :: LightBlack . write_bg (f) , Color :: LightRed => termion :: color :: LightRed . write_bg (f) , Color :: LightGreen => termion :: color :: LightGreen . write_bg (f) , Color :: LightBlue => termion :: color :: LightBlue . write_bg (f) , Color :: LightYellow => termion :: color :: LightYellow . write_bg (f) , Color :: LightMagenta => termion :: color :: LightMagenta . write_bg (f) , Color :: LightCyan => termion :: color :: LightCyan . write_bg (f) , Color :: White => termion :: color :: LightWhite . write_bg (f) , Color :: Indexed (i) => termion :: color :: AnsiValue (i) . write_bg (f) , Color :: Rgb (r , g , b) => termion :: color :: Rgb (r , g , b) . write_bg (f) , } } }
};
}
