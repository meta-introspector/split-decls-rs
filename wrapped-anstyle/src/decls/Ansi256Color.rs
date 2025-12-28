macro_rules! deps {
    () => {
        RgbColor!();
        AnsiColor!();
    };
}

macro_rules! Ansi256Color {
    () => {
        deps!();
        # [doc = " 256 (8-bit) color support"] # [doc = ""] # [doc = " - `0..16` are [`AnsiColor`] palette codes"] # [doc = " - `0..232` map to [`RgbColor`] color values"] # [doc = " - `232..` map to [`RgbColor`] gray-scale values"] # [allow (clippy :: exhaustive_structs)] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct Ansi256Color (pub u8) ;
    };
}

Ansi256Color!();