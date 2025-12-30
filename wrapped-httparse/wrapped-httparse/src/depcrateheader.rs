// Generated macro for Header (struct)
macro_rules! DepcrateHeader {
() => {
// Module: crate
// Provides: {"Header"}
// Dependencies: {}
# [doc = " Represents a parsed header."] # [derive (Copy , Clone , Eq , PartialEq)] pub struct Header < 'a > { # [doc = " The name portion of a header."] # [doc = ""] # [doc = " A header name must be valid ASCII-US, so it's safe to store as a `&str`."] pub name : & 'a str , # [doc = " The value portion of a header."] # [doc = ""] # [doc = " While headers **should** be ASCII-US, the specification allows for"] # [doc = " values that may not be, and so the value is stored as bytes."] pub value : & 'a [u8] , }
};
}
