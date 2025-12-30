// Generated macro for RawRef (struct)
macro_rules! DepcrateRawRef {
() => {
// Module: crate
// Provides: {"RawRef"}
// Dependencies: {}
# [doc = " Helper that allows both to encode and decode strings no matter whether they contain valid or"] # [doc = " invalid UTF-8."] # [doc = ""] # [doc = " Regardless of validity the UTF-8 content this type will always be serialized as a string."] # [derive (Clone , Copy , Debug , PartialEq)] # [doc (hidden)] pub struct RawRef < 'a > { s : Result < & 'a str , (& 'a [u8] , Utf8Error) > , }
};
}
