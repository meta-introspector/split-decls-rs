macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Display for Name { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Normal => write ! (f , "normal") , Self :: Default => write ! (f , "default") , Self :: Black => write ! (f , "black") , Self :: BrightBlack => write ! (f , "brightblack") , Self :: Red => write ! (f , "red") , Self :: BrightRed => write ! (f , "brightred") , Self :: Green => write ! (f , "green") , Self :: BrightGreen => write ! (f , "brightgreen") , Self :: Yellow => write ! (f , "yellow") , Self :: BrightYellow => write ! (f , "brightyellow") , Self :: Blue => write ! (f , "blue") , Self :: BrightBlue => write ! (f , "brightblue") , Self :: Magenta => write ! (f , "magenta") , Self :: BrightMagenta => write ! (f , "brightmagenta") , Self :: Cyan => write ! (f , "cyan") , Self :: BrightCyan => write ! (f , "brightcyan") , Self :: White => write ! (f , "white") , Self :: BrightWhite => write ! (f , "brightwhite") , Self :: Ansi (num) => num . fmt (f) , Self :: Rgb (r , g , b) => write ! (f , "#{r:02x}{g:02x}{b:02x}") , } } }
    };
}

impl_18!();