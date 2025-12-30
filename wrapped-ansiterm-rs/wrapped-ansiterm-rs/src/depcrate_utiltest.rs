// Generated macro for test (module)
macro_rules! Depcrate_utiltest {
() => {
// Module: crate::util
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: Colour :: * ; use crate :: display :: * ; use super :: * ; # [test] fn test () { let l = [Black . paint ("first") , Red . paint ("-second") , White . paint ("-third") ,] ; let a = ANSIStrings (& l) ; assert_eq ! (unstyle (& a) , "first-second-third") ; assert_eq ! (unstyled_len (& a) , 18) ; let l2 = [Black . paint ("st") , Red . paint ("-second") , White . paint ("-t") ,] ; assert_eq ! (sub_string (3 , 11 , & a) . as_slice () , & l2) ; } }
};
}
