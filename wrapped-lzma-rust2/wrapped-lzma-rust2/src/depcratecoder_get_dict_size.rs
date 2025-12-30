// Generated macro for coder_get_dict_size (function)
macro_rules! Depcratecoder_get_dict_size {
() => {
// Module: crate
// Provides: {"coder_get_dict_size"}
// Dependencies: {}
pub (crate) fn coder_get_dict_size (len : usize) -> usize { if len < DIST_STATES + MATCH_LEN_MIN { len - MATCH_LEN_MIN } else { DIST_STATES - 1 } }
};
}
