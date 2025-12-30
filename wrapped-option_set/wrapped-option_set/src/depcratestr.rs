// Generated macro for Str (enum)
macro_rules! DepcrateStr {
() => {
// Module: crate
// Provides: {"Str"}
// Dependencies: {}
# [doc = " Equivalent of `Cow<'a, str>` except that this"] # [doc = " type can deserialize from a borrowed string."] # [derive (Debug)] enum Str < 'a > { # [doc = " Owned."] String (String) , # [doc = " Borrowed."] Str (& 'a str) , }
};
}
