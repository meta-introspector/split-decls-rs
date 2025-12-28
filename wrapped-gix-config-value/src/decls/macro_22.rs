macro_rules! deps {
    () => {
        Color!();
    };
}

macro_rules! macro_22 {
    () => {
        deps!();
        bitflags :: bitflags ! { # [doc = " Discriminating enum for [`Color`] attributes."] # [doc = ""] # [doc = " `git-config` supports modifiers and their negators. The negating color"] # [doc = " attributes are equivalent to having a `no` or `no-` prefix to the normal"] # [doc = " variant."] # [derive (Default , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Attribute : u32 { const BOLD = 1 << 1 ; const DIM = 1 << 2 ; const ITALIC = 1 << 3 ; const UL = 1 << 4 ; const BLINK = 1 << 5 ; const REVERSE = 1 << 6 ; const STRIKE = 1 << 7 ; # [doc = " Reset is special as we have to be able to parse it, without git actually doing anything with it"] const RESET = 1 << 8 ; const NO_DIM = 1 << 21 ; const NO_BOLD = 1 << 22 ; const NO_ITALIC = 1 << 23 ; const NO_UL = 1 << 24 ; const NO_BLINK = 1 << 25 ; const NO_REVERSE = 1 << 26 ; const NO_STRIKE = 1 << 27 ; } }
    };
}

macro_22!();