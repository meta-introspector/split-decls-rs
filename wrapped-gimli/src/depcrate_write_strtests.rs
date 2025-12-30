// Generated macro for tests (module)
macro_rules! Depcrate_write_strtests {
() => {
// Module: crate::write::str
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "read")] mod tests { use super :: * ; use crate :: LittleEndian ; use crate :: read ; use crate :: write :: EndianVec ; # [test] fn test_string_table () { let mut strings = StringTable :: default () ; assert_eq ! (strings . count () , 0) ; let id1 = strings . add (& b"one" [..]) ; let id2 = strings . add (& b"two" [..]) ; let id3 = strings . add (& []) ; assert_eq ! (strings . add (& b"one" [..]) , id1) ; assert_eq ! (strings . add (& b"two" [..]) , id2) ; assert_eq ! (strings . add (& []) , id3) ; assert_eq ! (strings . get (id1) , & b"one" [..]) ; assert_eq ! (strings . get (id2) , & b"two" [..]) ; assert_eq ! (strings . get (id3) , & []) ; assert_eq ! (strings . count () , 3) ; assert_eq ! (strings . offset (id1) , DebugStrOffset (0)) ; assert_eq ! (strings . offset (id2) , DebugStrOffset (4)) ; assert_eq ! (strings . offset (id3) , DebugStrOffset (8)) ; let mut debug_str = DebugStr :: from (EndianVec :: new (LittleEndian)) ; strings . write (& mut debug_str) . unwrap () ; assert_eq ! (debug_str . slice () , b"one\0two\0\0") ; let read_debug_str = read :: DebugStr :: new (debug_str . slice () , LittleEndian) ; let str1 = read_debug_str . get_str (strings . offset (id1)) . unwrap () ; let str2 = read_debug_str . get_str (strings . offset (id2)) . unwrap () ; let str3 = read_debug_str . get_str (strings . offset (id3)) . unwrap () ; assert_eq ! (str1 . slice () , & b"one" [..]) ; assert_eq ! (str2 . slice () , & b"two" [..]) ; assert_eq ! (str3 . slice () , b"") ; } }
};
}
