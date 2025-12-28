macro_rules! deps {
    () => {
        DisplayBuffer!();
        RgbColor!();
        Style!();
        Color!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl RgbColor { # [doc = " Create a [`Style`][crate::Style] with this as the foreground"] # [inline] pub fn on (self , background : impl Into < Color >) -> crate :: Style { crate :: Style :: new () . fg_color (Some (self . into ())) . bg_color (Some (background . into ())) } # [doc = " Create a [`Style`][crate::Style] with this as the foreground"] # [inline] pub const fn on_default (self) -> crate :: Style { crate :: Style :: new () . fg_color (Some (Color :: Rgb (self))) } # [doc = " Red"] # [inline] pub const fn r (self) -> u8 { self . 0 } # [doc = " Green"] # [inline] pub const fn g (self) -> u8 { self . 1 } # [doc = " Blue"] # [inline] pub const fn b (self) -> u8 { self . 2 } # [doc = " Render the ANSI code for a foreground color"] # [inline] pub fn render_fg (self) -> impl core :: fmt :: Display + Copy { self . as_fg_buffer () } # [inline] fn as_fg_buffer (& self) -> DisplayBuffer { DisplayBuffer :: default () . write_str ("\x1B[38;2;") . write_code (self . r ()) . write_str (";") . write_code (self . g ()) . write_str (";") . write_code (self . b ()) . write_str ("m") } # [doc = " Render the ANSI code for a background color"] # [inline] pub fn render_bg (self) -> impl core :: fmt :: Display + Copy { self . as_bg_buffer () } # [inline] fn as_bg_buffer (& self) -> DisplayBuffer { DisplayBuffer :: default () . write_str ("\x1B[48;2;") . write_code (self . r ()) . write_str (";") . write_code (self . g ()) . write_str (";") . write_code (self . b ()) . write_str ("m") } # [inline] fn as_underline_buffer (& self) -> DisplayBuffer { DisplayBuffer :: default () . write_str ("\x1B[58;2;") . write_code (self . r ()) . write_str (";") . write_code (self . g ()) . write_str (";") . write_code (self . b ()) . write_str ("m") } }
    };
}

impl_16!()