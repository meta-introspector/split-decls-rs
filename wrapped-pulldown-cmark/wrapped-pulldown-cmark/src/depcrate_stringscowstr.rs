// Generated macro for CowStr (enum)
macro_rules! Depcrate_stringsCowStr {
() => {
// Module: crate::strings
// Provides: {"CowStr"}
// Dependencies: {}
# [doc = " A copy-on-write string that can be owned, borrowed"] # [doc = " or inlined."] # [doc = ""] # [doc = " It is three words long."] # [derive (Debug , Eq)] pub enum CowStr < 'a > { # [doc = " An owned, immutable string."] Boxed (Box < str >) , # [doc = " A borrowed string."] Borrowed (& 'a str) , # [doc = " A short inline string."] Inlined (InlineStr) , }
};
}
