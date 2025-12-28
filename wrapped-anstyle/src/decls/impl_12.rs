macro_rules! deps {
    () => {
        AnsiColor!();
        Style!();
        Ansi256Color!();
        DisplayBuffer!();
        Color!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Ansi256Color { # [doc = " Create a [`Style`][crate::Style] with this as the foreground"] # [inline] pub fn on (self , background : impl Into < Color >) -> crate :: Style { crate :: Style :: new () . fg_color (Some (self . into ())) . bg_color (Some (background . into ())) } # [doc = " Create a [`Style`][crate::Style] with this as the foreground"] # [inline] pub const fn on_default (self) -> crate :: Style { crate :: Style :: new () . fg_color (Some (Color :: Ansi256 (self))) } # [doc = " Get the raw value"] # [inline] pub const fn index (self) -> u8 { self . 0 } # [doc = " Convert to [`AnsiColor`] when there is a 1:1 mapping"] # [inline] pub const fn into_ansi (self) -> Option < AnsiColor > { match self . index () { 0 => Some (AnsiColor :: Black) , 1 => Some (AnsiColor :: Red) , 2 => Some (AnsiColor :: Green) , 3 => Some (AnsiColor :: Yellow) , 4 => Some (AnsiColor :: Blue) , 5 => Some (AnsiColor :: Magenta) , 6 => Some (AnsiColor :: Cyan) , 7 => Some (AnsiColor :: White) , 8 => Some (AnsiColor :: BrightBlack) , 9 => Some (AnsiColor :: BrightRed) , 10 => Some (AnsiColor :: BrightGreen) , 11 => Some (AnsiColor :: BrightYellow) , 12 => Some (AnsiColor :: BrightBlue) , 13 => Some (AnsiColor :: BrightMagenta) , 14 => Some (AnsiColor :: BrightCyan) , 15 => Some (AnsiColor :: BrightWhite) , _ => None , } } # [doc = " Losslessly convert from [`AnsiColor`]"] # [inline] pub const fn from_ansi (color : AnsiColor) -> Self { match color { AnsiColor :: Black => Self (0) , AnsiColor :: Red => Self (1) , AnsiColor :: Green => Self (2) , AnsiColor :: Yellow => Self (3) , AnsiColor :: Blue => Self (4) , AnsiColor :: Magenta => Self (5) , AnsiColor :: Cyan => Self (6) , AnsiColor :: White => Self (7) , AnsiColor :: BrightBlack => Self (8) , AnsiColor :: BrightRed => Self (9) , AnsiColor :: BrightGreen => Self (10) , AnsiColor :: BrightYellow => Self (11) , AnsiColor :: BrightBlue => Self (12) , AnsiColor :: BrightMagenta => Self (13) , AnsiColor :: BrightCyan => Self (14) , AnsiColor :: BrightWhite => Self (15) , } } # [doc = " Render the ANSI code for a foreground color"] # [inline] pub fn render_fg (self) -> impl core :: fmt :: Display + Copy { self . as_fg_buffer () } # [inline] fn as_fg_buffer (& self) -> DisplayBuffer { DisplayBuffer :: default () . write_str ("\x1B[38;5;") . write_code (self . index ()) . write_str ("m") } # [doc = " Render the ANSI code for a background color"] # [inline] pub fn render_bg (self) -> impl core :: fmt :: Display + Copy { self . as_bg_buffer () } # [inline] fn as_bg_buffer (& self) -> DisplayBuffer { DisplayBuffer :: default () . write_str ("\x1B[48;5;") . write_code (self . index ()) . write_str ("m") } # [inline] fn as_underline_buffer (& self) -> DisplayBuffer { DisplayBuffer :: default () . write_str ("\x1B[58;5;") . write_code (self . index ()) . write_str ("m") } }
    };
}

impl_12!();