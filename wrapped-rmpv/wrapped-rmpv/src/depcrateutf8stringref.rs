// Generated macro for Utf8StringRef (struct)
macro_rules! DepcrateUtf8StringRef {
() => {
// Module: crate
// Provides: {"Utf8StringRef"}
// Dependencies: {}
# [doc = " A non-owning evil twin of `Utf8String`. Does exactly the same thing except ownership."] # [derive (Clone , Copy , Debug , PartialEq)] pub struct Utf8StringRef < 'a > { s : Result < & 'a str , (& 'a [u8] , Utf8Error) > , }
};
}
