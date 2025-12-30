// Generated macro for impl_53 (impl)
macro_rules! Depcrate_dfa_denseimpl_53 {
() => {
// Module: crate::dfa::dense
// Provides: {"impl_53"}
// Dependencies: {}
# [cfg (feature = "dfa-build")] impl MatchStates < Vec < u32 > > { fn empty (pattern_len : usize) -> MatchStates < Vec < u32 > > { assert ! (pattern_len <= PatternID :: LIMIT) ; MatchStates { slices : vec ! [] , pattern_ids : vec ! [] , pattern_len } } fn new (matches : & BTreeMap < StateID , Vec < PatternID > > , pattern_len : usize ,) -> Result < MatchStates < Vec < u32 > > , BuildError > { let mut m = MatchStates :: empty (pattern_len) ; for (_ , pids) in matches . iter () { let start = PatternID :: new (m . pattern_ids . len ()) . map_err (| _ | BuildError :: too_many_match_pattern_ids ()) ? ; m . slices . push (start . as_u32 ()) ; m . slices . push (u32 :: try_from (pids . len ()) . unwrap ()) ; for & pid in pids { m . pattern_ids . push (pid . as_u32 ()) ; } } m . pattern_len = pattern_len ; Ok (m) } fn new_with_map (& self , matches : & BTreeMap < StateID , Vec < PatternID > > ,) -> Result < MatchStates < Vec < u32 > > , BuildError > { MatchStates :: new (matches , self . pattern_len) } }
};
}
