// Generated macro for YearBounds (struct)
macro_rules! Depcrate_chinese_basedYearBounds {
() => {
// Module: crate::chinese_based
// Provides: {"YearBounds"}
// Dependencies: {}
# [doc = " Marks the bounds of a lunar year"] # [derive (Debug , Copy , Clone)] # [allow (clippy :: exhaustive_structs)] pub struct YearBounds { # [doc = " The date marking the start of the current lunar year"] pub new_year : RataDie , # [doc = " The date marking the start of the next lunar year"] pub next_new_year : RataDie , }
};
}
