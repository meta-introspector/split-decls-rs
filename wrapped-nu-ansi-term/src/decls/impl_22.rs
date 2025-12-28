macro_rules! deps {
    () => {
        Style!();
        Color!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl From < Color > for Style { # [doc = " You can turn a `Color` into a `Style` with the foreground color set"] # [doc = " with the `From` trait."] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::{Style, Color};"] # [doc = " let green_foreground = Style::default().fg(Color::Green);"] # [doc = " assert_eq!(green_foreground, Color::Green.normal());"] # [doc = " assert_eq!(green_foreground, Color::Green.into());"] # [doc = " assert_eq!(green_foreground, Style::from(Color::Green));"] # [doc = " ```"] fn from (color : Color) -> Style { color . normal () } }
    };
}

impl_22!();