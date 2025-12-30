// Generated macro for impl_95 (impl)
macro_rules! Depcrate_astimpl_95 {
() => {
// Module: crate::ast
// Provides: {"impl_95"}
// Dependencies: {}
impl ClassAsciiKind { # [doc = " Return the corresponding ClassAsciiKind variant for the given name."] # [doc = ""] # [doc = " The name given should correspond to the lowercase version of the"] # [doc = " variant name. e.g., `cntrl` is the name for `ClassAsciiKind::Cntrl`."] # [doc = ""] # [doc = " If no variant with the corresponding name exists, then `None` is"] # [doc = " returned."] pub fn from_name (name : & str) -> Option < ClassAsciiKind > { use self :: ClassAsciiKind :: * ; match name { "alnum" => Some (Alnum) , "alpha" => Some (Alpha) , "ascii" => Some (Ascii) , "blank" => Some (Blank) , "cntrl" => Some (Cntrl) , "digit" => Some (Digit) , "graph" => Some (Graph) , "lower" => Some (Lower) , "print" => Some (Print) , "punct" => Some (Punct) , "space" => Some (Space) , "upper" => Some (Upper) , "word" => Some (Word) , "xdigit" => Some (Xdigit) , _ => None , } } }
};
}
