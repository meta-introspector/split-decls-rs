// Generated macro for extended_from_iso (function)
macro_rules! Depcrate_chinese_basedextended_from_iso {
() => {
// Module: crate::chinese_based
// Provides: {"extended_from_iso"}
// Dependencies: {}
# [doc = " Given an ISO year, return the extended year"] # [deprecated (since = "0.2.3" , note = "extended year calculation subject to removal")] pub fn extended_from_iso < C : ChineseBased > (iso_year : i32) -> i32 { iso_year - const { let Ok (y) = crate :: gregorian :: year_from_fixed (C :: EPOCH) else { panic ! () } ; y - 1 } }
};
}
