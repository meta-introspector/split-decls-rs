// Generated macro for tests (module)
macro_rules! Depcrate_string_draintests {
() => {
// Module: crate::string::drain
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: String ; # [test] fn drain_front () { let mut s = String :: < 8 > :: try_from ("abcd") . unwrap () ; let mut it = s . drain (.. 1) ; assert_eq ! (it . next () , Some ('a')) ; drop (it) ; assert_eq ! (s , "bcd") ; } # [test] fn drain_middle () { let mut s = String :: < 8 > :: try_from ("abcd") . unwrap () ; let mut it = s . drain (1 .. 3) ; assert_eq ! (it . next () , Some ('b')) ; assert_eq ! (it . next () , Some ('c')) ; drop (it) ; assert_eq ! (s , "ad") ; } # [test] fn drain_end () { let mut s = String :: < 8 > :: try_from ("abcd") . unwrap () ; let mut it = s . drain (3 ..) ; assert_eq ! (it . next () , Some ('d')) ; drop (it) ; assert_eq ! (s , "abc") ; } }
};
}
