// Generated macro for impl_150 (impl)
macro_rules! Depcrate_dfa_sparseimpl_150 {
() => {
// Module: crate::dfa::sparse
// Provides: {"impl_150"}
// Dependencies: {}
# [cfg (feature = "dfa-build")] impl StartTable < Vec < u8 > > { fn new < T : AsRef < [u32] > > (dfa : & dense :: DFA < T > , pattern_len : Option < usize > ,) -> StartTable < Vec < u8 > > { let stride = Start :: len () ; let len = stride . checked_mul (pattern_len . unwrap_or (0)) . unwrap () . checked_add (stride . checked_mul (2) . unwrap ()) . unwrap () . checked_mul (StateID :: SIZE) . unwrap () ; StartTable { table : vec ! [0 ; len] , kind : dfa . start_kind () , start_map : dfa . start_map () . clone () , stride , pattern_len , universal_start_unanchored : dfa . universal_start_state (Anchored :: No) , universal_start_anchored : dfa . universal_start_state (Anchored :: Yes) , } } fn from_dense_dfa < T : AsRef < [u32] > > (dfa : & dense :: DFA < T > , remap : & [StateID] ,) -> Result < StartTable < Vec < u8 > > , BuildError > { let start_pattern_len = if dfa . starts_for_each_pattern () { Some (dfa . pattern_len ()) } else { None } ; let mut sl = StartTable :: new (dfa , start_pattern_len) ; for (old_start_id , anchored , sty) in dfa . starts () { let new_start_id = remap [dfa . to_index (old_start_id)] ; sl . set_start (anchored , sty , new_start_id) ; } Ok (sl) } }
};
}
