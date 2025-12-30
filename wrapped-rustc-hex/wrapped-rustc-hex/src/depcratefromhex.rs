// Generated macro for FromHex (trait)
macro_rules! DepcrateFromHex {
() => {
// Module: crate
// Provides: {"FromHex"}
// Dependencies: {}
# [doc = " A from-hex conversion trait."] pub trait FromHex { # [doc = " Converts the value of `self`, interpreted as hexadecimal encoded data,"] # [doc = " into an owned value constructed from an iterator of bytes."] fn from_hex < T : FromIterator < u8 > > (& self) -> Result < T , FromHexError > ; }
};
}
