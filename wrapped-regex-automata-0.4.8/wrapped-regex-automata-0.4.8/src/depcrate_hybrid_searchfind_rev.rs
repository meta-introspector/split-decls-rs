// Generated macro for find_rev (function)
macro_rules! Depcrate_hybrid_searchfind_rev {
() => {
// Module: crate::hybrid::search
// Provides: {"find_rev"}
// Dependencies: {}
# [inline (never)] pub (crate) fn find_rev (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > ,) -> Result < Option < HalfMatch > , MatchError > { if input . is_done () { return Ok (None) ; } if input . get_earliest () { find_rev_imp (dfa , cache , input , true) } else { find_rev_imp (dfa , cache , input , false) } }
};
}
