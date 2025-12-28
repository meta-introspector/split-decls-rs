macro_rules! deps {
    () => {
        Rgb!();
        Color!();
        AnyWrite!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Color { fn write_foreground_code < W : AnyWrite + ? Sized > (& self , f : & mut W) -> Result < () , W :: Error > { match self { Color :: Black => write ! (f , "30") , Color :: Red => write ! (f , "31") , Color :: Green => write ! (f , "32") , Color :: Yellow => write ! (f , "33") , Color :: Blue => write ! (f , "34") , Color :: Purple => write ! (f , "35") , Color :: Magenta => write ! (f , "35") , Color :: Cyan => write ! (f , "36") , Color :: White => write ! (f , "37") , Color :: Fixed (num) => write ! (f , "38;5;{}" , num) , Color :: Rgb (r , g , b) => write ! (f , "38;2;{};{};{}" , r , g , b) , Color :: Default => write ! (f , "39") , Color :: DarkGray => write ! (f , "90") , Color :: LightRed => write ! (f , "91") , Color :: LightGreen => write ! (f , "92") , Color :: LightYellow => write ! (f , "93") , Color :: LightBlue => write ! (f , "94") , Color :: LightPurple => write ! (f , "95") , Color :: LightMagenta => write ! (f , "95") , Color :: LightCyan => write ! (f , "96") , Color :: LightGray => write ! (f , "97") , } } fn write_background_code < W : AnyWrite + ? Sized > (& self , f : & mut W) -> Result < () , W :: Error > { match self { Color :: Black => write ! (f , "40") , Color :: Red => write ! (f , "41") , Color :: Green => write ! (f , "42") , Color :: Yellow => write ! (f , "43") , Color :: Blue => write ! (f , "44") , Color :: Purple => write ! (f , "45") , Color :: Magenta => write ! (f , "45") , Color :: Cyan => write ! (f , "46") , Color :: White => write ! (f , "47") , Color :: Fixed (num) => write ! (f , "48;5;{}" , num) , Color :: Rgb (r , g , b) => write ! (f , "48;2;{};{};{}" , r , g , b) , Color :: Default => write ! (f , "49") , Color :: DarkGray => write ! (f , "100") , Color :: LightRed => write ! (f , "101") , Color :: LightGreen => write ! (f , "102") , Color :: LightYellow => write ! (f , "103") , Color :: LightBlue => write ! (f , "104") , Color :: LightPurple => write ! (f , "105") , Color :: LightMagenta => write ! (f , "105") , Color :: LightCyan => write ! (f , "106") , Color :: LightGray => write ! (f , "107") , } } }
    };
}

impl_4!()