// Generated macro for test (function)
macro_rules! Depcrate_lazy_automatontest {
() => {
// Module: crate::lazy_automaton
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] # [test] fn test () { use crate :: provider :: SerdeDFA ; use regex_automata :: Input ; use std :: borrow :: Cow ; let matcher = SerdeDFA :: new (Cow :: Borrowed ("^11(000)*$")) . unwrap () ; for writeable in [1i32 , 11 , 110 , 11000 , 211000] { assert_eq ! (matcher . deref () . try_search_fwd (& Input :: new (writeable . write_to_string () . as_bytes ()) . anchored (regex_automata :: Anchored :: Yes)) . unwrap () . is_some () , matcher . deref () . matches_earliest_fwd_lazy (& writeable)) ; } struct ExitEarlyTest ; impl writeable :: Writeable for ExitEarlyTest { fn write_to < W : core :: fmt :: Write + ? Sized > (& self , sink : & mut W) -> core :: fmt :: Result { sink . write_str ("12") ? ; unreachable ! () } } assert ! (! matcher . deref () . matches_earliest_fwd_lazy (& ExitEarlyTest)) ; }
};
}
