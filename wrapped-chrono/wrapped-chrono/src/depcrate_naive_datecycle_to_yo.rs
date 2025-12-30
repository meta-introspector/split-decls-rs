// Generated macro for cycle_to_yo (function)
macro_rules! Depcrate_naive_datecycle_to_yo {
() => {
// Module: crate::naive::date
// Provides: {"cycle_to_yo"}
// Dependencies: {}
const fn cycle_to_yo (cycle : u32) -> (u32 , u32) { let mut year_mod_400 = cycle / 365 ; let mut ordinal0 = cycle % 365 ; let delta = YEAR_DELTAS [year_mod_400 as usize] as u32 ; if ordinal0 < delta { year_mod_400 -= 1 ; ordinal0 += 365 - YEAR_DELTAS [year_mod_400 as usize] as u32 ; } else { ordinal0 -= delta ; } (year_mod_400 , ordinal0 + 1) }
};
}
