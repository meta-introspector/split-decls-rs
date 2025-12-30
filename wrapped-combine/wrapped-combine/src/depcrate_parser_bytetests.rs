// Generated macro for tests (module)
macro_rules! Depcrate_parser_bytetests {
() => {
// Module: crate::parser::byte
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (feature = "std" , test))] mod tests { use crate :: stream :: { buffered , position , read } ; use super :: * ; # [test] fn memslice_basic () { let haystack = b"abc123" ; assert_eq ! (memslice (b"" , haystack) , Some (0)) ; assert_eq ! (memslice (b"a" , haystack) , Some (0)) ; assert_eq ! (memslice (b"ab" , haystack) , Some (0)) ; assert_eq ! (memslice (b"c12" , haystack) , Some (2)) ; let haystack2 = b"abcab2" ; assert_eq ! (memslice (b"abc" , haystack2) , Some (0)) ; assert_eq ! (memslice (b"ab2" , haystack2) , Some (3)) ; let haystack3 = b"aaabaaaa" ; assert_eq ! (memslice (b"aaaa" , haystack3) , Some (4)) ; } # [test] fn bytes_read_stream () { assert ! (bytes (b"abc") . parse (buffered :: Stream :: new (position :: Stream :: new (read :: Stream :: new ("abc" . as_bytes ())) , 1)) . is_ok ()) ; } }
};
}
