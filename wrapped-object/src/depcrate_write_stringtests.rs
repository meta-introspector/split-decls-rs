// Generated macro for tests (module)
macro_rules! Depcrate_write_stringtests {
() => {
// Module: crate::write::string
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn string_table () { let mut table = StringTable :: default () ; let id0 = table . add (b"") ; let id1 = table . add (b"foo") ; let id2 = table . add (b"bar") ; let id3 = table . add (b"foobar") ; let mut data = Vec :: new () ; data . push (0) ; table . write (1 , & mut data) ; assert_eq ! (data , b"\0foobar\0foo\0") ; assert_eq ! (table . get_offset (id0) , 11) ; assert_eq ! (table . get_offset (id1) , 8) ; assert_eq ! (table . get_offset (id2) , 4) ; assert_eq ! (table . get_offset (id3) , 1) ; } }
};
}
