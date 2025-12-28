macro_rules! deps {
    () => {
        Color!();
    };
}

macro_rules! Name {
    () => {
        deps!();
        # [doc = " Discriminating enum for names of [`Color`] values."] # [doc = ""] # [doc = " `git-config` supports the eight standard colors, their bright variants, an"] # [doc = " ANSI color code, or a 24-bit hex value prefixed with an octothorpe/hash."] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] # [allow (missing_docs)] pub enum Name { Normal , Default , Black , BrightBlack , Red , BrightRed , Green , BrightGreen , Yellow , BrightYellow , Blue , BrightBlue , Magenta , BrightMagenta , Cyan , BrightCyan , White , BrightWhite , Ansi (u8) , Rgb (u8 , u8 , u8) , }
    };
}

Name!();