macro_rules! deps {
    () => {
        RgbColor!();
        Ansi256Color!();
        AnsiColor!();
    };
}

macro_rules! Color {
    () => {
        deps!();
        # [doc = " Any ANSI color code scheme"] # [allow (clippy :: exhaustive_enums)] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum Color { # [doc = " Available 4-bit ANSI color palette codes"] # [doc = ""] # [doc = " The user's terminal defines the meaning of each palette code."] Ansi (AnsiColor) , # [doc = " 256 (8-bit) color support"] # [doc = ""] # [doc = " - `0..16` are [`AnsiColor`] palette codes"] # [doc = " - `0..232` map to [`RgbColor`] color values"] # [doc = " - `232..` map to [`RgbColor`] gray-scale values"] Ansi256 (Ansi256Color) , # [doc = " 24-bit ANSI RGB color codes"] Rgb (RgbColor) , }
    };
}

Color!()