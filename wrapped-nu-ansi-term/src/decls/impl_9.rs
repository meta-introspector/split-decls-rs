macro_rules! deps {
    () => {
        Color!();
        Suffix!();
        Style!();
        Prefix!();
        Infix!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Color { # [doc = " The prefix bytes for this color as a `Style`. These are the bytes"] # [doc = " that tell the terminal to use a different color or font style."] # [doc = ""] # [doc = " See also [`Style::prefix`](struct.Style.html#method.prefix)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::Color::Green;"] # [doc = ""] # [doc = " assert_eq!(\"\\x1b[32m\","] # [doc = "            Green.prefix().to_string());"] # [doc = " ```"] pub fn prefix (self) -> Prefix { Prefix (self . normal ()) } # [doc = " The infix bytes between this color and `next` color. These are the bytes"] # [doc = " that tell the terminal to use the `next` color, or to do nothing if"] # [doc = " the two colors are equal."] # [doc = ""] # [doc = " See also [`Style::infix`](struct.Style.html#method.infix)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::Color::{Red, Yellow};"] # [doc = ""] # [doc = " assert_eq!(\"\\x1b[33m\","] # [doc = "            Red.infix(Yellow).to_string());"] # [doc = " ```"] pub fn infix (self , next : Color) -> Infix { Infix (self . normal () , next . normal ()) } # [doc = " The suffix for this color as a `Style`. These are the bytes that"] # [doc = " tell the terminal to reset back to its normal color and font style."] # [doc = ""] # [doc = " See also [`Style::suffix`](struct.Style.html#method.suffix)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::Color::Purple;"] # [doc = ""] # [doc = " assert_eq!(\"\\x1b[0m\","] # [doc = "            Purple.suffix().to_string());"] # [doc = " ```"] pub fn suffix (self) -> Suffix { Suffix (self . normal ()) } }
    };
}

impl_9!();