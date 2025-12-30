// Generated macro for ToHex (trait)
macro_rules! DepcrateToHex {
() => {
// Module: crate
// Provides: {"ToHex"}
// Dependencies: {}
# [doc = " A trait for converting a value to hexadecimal encoding"] pub trait ToHex { # [doc = " Converts the value of `self` to a hex value, constructed from"] # [doc = " an iterator of characters."] fn to_hex < T : FromIterator < char > > (& self) -> T ; }
};
}
