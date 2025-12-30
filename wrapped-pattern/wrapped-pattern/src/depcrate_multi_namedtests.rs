// Generated macro for tests (module)
macro_rules! Depcrate_multi_namedtests {
() => {
// Module: crate::multi_named
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: { MultiNamedPlaceholder , MultiNamedPlaceholderPattern } ; # [test] fn test_invalid () { let long_str = "0123456789" . repeat (1000000) ; let strings = ["{" , "{@}" , "\x00" , "\x07" ,] ; for string in strings { let string = string . replace ('@' , & long_str) ; assert ! (MultiNamedPlaceholderPattern :: try_from_str (& string , Default :: default ()) . is_err () , "{string:?}") ; } let stores = ["\x00" , "\x02" , "\x00\x02" , "\x00\x02a" ,] ; for store in stores { assert ! (MultiNamedPlaceholder :: validate_store (store) . is_err () , "{store:?}") ; } } }
};
}
