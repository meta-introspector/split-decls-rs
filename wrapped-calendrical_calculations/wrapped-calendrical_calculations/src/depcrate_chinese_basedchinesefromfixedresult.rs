// Generated macro for ChineseFromFixedResult (struct)
macro_rules! Depcrate_chinese_basedChineseFromFixedResult {
() => {
// Module: crate::chinese_based
// Provides: {"ChineseFromFixedResult"}
// Dependencies: {}
# [doc = " chinese_based_date_from_fixed returns extra things for use in caching"] # [derive (Debug)] # [non_exhaustive] pub struct ChineseFromFixedResult { # [doc = " The chinese year"] pub year : i32 , # [doc = " The chinese month"] pub month : u8 , # [doc = " The chinese day"] pub day : u8 , # [doc = " The bounds of the current lunar year"] pub year_bounds : YearBounds , # [doc = " The index of the leap month, if any"] pub leap_month : Option < NonZeroU8 > , }
};
}
