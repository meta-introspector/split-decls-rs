// Generated macro for iso_from_extended (function)
macro_rules! Depcrate_chinese_basediso_from_extended {
() => {
// Module: crate::chinese_based
// Provides: {"iso_from_extended"}
// Dependencies: {}
# [doc = " Given an extended year, return the ISO year"] # [deprecated (since = "0.2.3" , note = "extended year calculation subject to removal")] pub fn iso_from_extended < C : ChineseBased > (extended_year : i32) -> i32 { extended_year + const { let Ok (y) = crate :: gregorian :: year_from_fixed (C :: EPOCH) else { panic ! () } ; y - 1 } }
};
}
