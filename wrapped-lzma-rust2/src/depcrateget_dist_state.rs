// Generated macro for get_dist_state (function)
macro_rules! Depcrateget_dist_state {
() => {
// Module: crate
// Provides: {"get_dist_state"}
// Dependencies: {}
pub (crate) fn get_dist_state (len : u32) -> u32 { (if (len as usize) < DIST_STATES + MATCH_LEN_MIN { len as usize - MATCH_LEN_MIN } else { DIST_STATES - 1 }) as u32 }
};
}
