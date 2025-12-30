// Generated macro for tests (module)
macro_rules! Depcrate_dfa_automatontests {
() => {
// Module: crate::dfa::automaton
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "syntax" , feature = "dfa-build"))] mod tests { # [test] fn object_safe () { use crate :: { dfa :: { dense , Automaton } , HalfMatch , Input , } ; let dfa = dense :: DFA :: new ("abc") . unwrap () ; let dfa : & dyn Automaton = & dfa ; assert_eq ! (Ok (Some (HalfMatch :: must (0 , 6))) , dfa . try_search_fwd (& Input :: new (b"xyzabcxyz")) ,) ; } }
};
}
