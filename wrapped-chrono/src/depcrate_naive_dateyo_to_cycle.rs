// Generated macro for yo_to_cycle (function)
macro_rules! Depcrate_naive_dateyo_to_cycle {
() => {
// Module: crate::naive::date
// Provides: {"yo_to_cycle"}
// Dependencies: {}
const fn yo_to_cycle (year_mod_400 : u32 , ordinal : u32) -> u32 { year_mod_400 * 365 + YEAR_DELTAS [year_mod_400 as usize] as u32 + ordinal - 1 }
};
}
