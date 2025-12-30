// Generated macro for impl_156 (impl)
macro_rules! Depcrate_dfa_sparseimpl_156 {
() => {
// Module: crate::dfa::sparse
// Provides: {"impl_156"}
// Dependencies: {}
# [cfg (feature = "dfa-build")] impl < T : AsMut < [u8] > > StartTable < T > { # [doc = " Set the start state for the given index and pattern."] # [doc = ""] # [doc = " If the pattern ID or state ID are not valid, then this will panic."] fn set_start (& mut self , anchored : Anchored , start : Start , id : StateID) { let start_index = start . as_usize () ; let index = match anchored { Anchored :: No => start_index , Anchored :: Yes => self . stride + start_index , Anchored :: Pattern (pid) => { let pid = pid . as_usize () ; let len = self . pattern_len . expect ("start states for each pattern enabled") ; assert ! (pid < len , "invalid pattern ID {pid:?}") ; self . stride . checked_mul (pid) . unwrap () . checked_add (self . stride . checked_mul (2) . unwrap ()) . unwrap () . checked_add (start_index) . unwrap () } } ; let start = index * StateID :: SIZE ; let end = start + StateID :: SIZE ; wire :: write_state_id :: < wire :: NE > (id , & mut self . table . as_mut () [start .. end] ,) ; } }
};
}
