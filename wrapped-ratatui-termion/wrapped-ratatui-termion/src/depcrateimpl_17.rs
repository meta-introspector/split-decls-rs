// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl fmt :: Display for Fg { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { Color :: Reset => termion :: color :: Reset . write_fg (f) , Color :: Black => termion :: color :: Black . write_fg (f) , Color :: Red => termion :: color :: Red . write_fg (f) , Color :: Green => termion :: color :: Green . write_fg (f) , Color :: Yellow => termion :: color :: Yellow . write_fg (f) , Color :: Blue => termion :: color :: Blue . write_fg (f) , Color :: Magenta => termion :: color :: Magenta . write_fg (f) , Color :: Cyan => termion :: color :: Cyan . write_fg (f) , Color :: Gray => termion :: color :: White . write_fg (f) , Color :: DarkGray => termion :: color :: LightBlack . write_fg (f) , Color :: LightRed => termion :: color :: LightRed . write_fg (f) , Color :: LightGreen => termion :: color :: LightGreen . write_fg (f) , Color :: LightBlue => termion :: color :: LightBlue . write_fg (f) , Color :: LightYellow => termion :: color :: LightYellow . write_fg (f) , Color :: LightMagenta => termion :: color :: LightMagenta . write_fg (f) , Color :: LightCyan => termion :: color :: LightCyan . write_fg (f) , Color :: White => termion :: color :: LightWhite . write_fg (f) , Color :: Indexed (i) => termion :: color :: AnsiValue (i) . write_fg (f) , Color :: Rgb (r , g , b) => termion :: color :: Rgb (r , g , b) . write_fg (f) , } } }
};
}
