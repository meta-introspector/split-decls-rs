// Generated macro for Raw (struct)
macro_rules! DepcrateRaw {
() => {
// Module: crate
// Provides: {"Raw"}
// Dependencies: {}
# [doc = " Helper that allows both to encode and decode strings no matter whether they contain valid or"] # [doc = " invalid UTF-8."] # [doc = ""] # [doc = " Regardless of validity the UTF-8 content this type will always be serialized as a string."] # [derive (Clone , Debug , PartialEq)] # [doc (hidden)] pub struct Raw { s : Result < String , (Vec < u8 > , Utf8Error) > , }
};
}
