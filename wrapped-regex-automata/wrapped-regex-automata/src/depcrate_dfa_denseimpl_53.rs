// Generated macro for impl_53 (impl)
macro_rules! Depcrate_dfa_denseimpl_53 {
() => {
// Module: crate::dfa::dense
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a > MatchStates < & 'a [u32] > { unsafe fn from_bytes_unchecked (mut slice : & 'a [u8] ,) -> Result < (MatchStates < & 'a [u32] > , usize) , DeserializeError > { let slice_start = slice . as_ptr () . as_usize () ; let (state_len , nr) = wire :: try_read_u32_as_usize (slice , "match state length") ? ; slice = & slice [nr ..] ; let pair_len = wire :: mul (2 , state_len , "match state offset pairs") ? ; let slices_bytes_len = wire :: mul (pair_len , PatternID :: SIZE , "match state slice offset byte length" ,) ? ; wire :: check_slice_len (slice , slices_bytes_len , "match state slices") ? ; wire :: check_alignment :: < PatternID > (slice) ? ; let slices_bytes = & slice [.. slices_bytes_len] ; slice = & slice [slices_bytes_len ..] ; let slices = core :: slice :: from_raw_parts (slices_bytes . as_ptr () . cast :: < u32 > () , pair_len ,) ; let (pattern_len , nr) = wire :: try_read_u32_as_usize (slice , "pattern length") ? ; slice = & slice [nr ..] ; let (idlen , nr) = wire :: try_read_u32_as_usize (slice , "pattern ID length") ? ; slice = & slice [nr ..] ; let pattern_ids_len = wire :: mul (idlen , PatternID :: SIZE , "pattern ID byte length") ? ; wire :: check_slice_len (slice , pattern_ids_len , "match pattern IDs") ? ; wire :: check_alignment :: < PatternID > (slice) ? ; let pattern_ids_bytes = & slice [.. pattern_ids_len] ; slice = & slice [pattern_ids_len ..] ; let pattern_ids = core :: slice :: from_raw_parts (pattern_ids_bytes . as_ptr () . cast :: < u32 > () , idlen ,) ; let ms = MatchStates { slices , pattern_ids , pattern_len } ; Ok ((ms , slice . as_ptr () . as_usize () - slice_start)) } }
};
}
