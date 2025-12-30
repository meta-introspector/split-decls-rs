// Generated macro for Mdf (struct)
macro_rules! Depcrate_naive_internalsMdf {
() => {
// Module: crate::naive::internals
// Provides: {"Mdf"}
// Dependencies: {}
# [doc = " Month, day of month and year flags: `(month << 9) | (day << 4) | flags`"] # [doc = " `M_MMMD_DDDD_LFFF`"] # [doc = ""] # [doc = " The whole bits except for the least 3 bits are referred as `Mdl` (month, day of month, and leap"] # [doc = " year flag), which is an index to the `MDL_TO_OL` lookup table."] # [doc = ""] # [doc = " The conversion between the packed calendar date (`Mdf`) and the ordinal date (`NaiveDate`) is"] # [doc = " based on the moderately-sized lookup table (~1.5KB) and the packed representation is chosen for"] # [doc = " efficient lookup."] # [doc = ""] # [doc = " The methods of `Mdf` validate their inputs as late as possible. Dates that can't exist, like"] # [doc = " February 30, can still be represented. This allows the validation to be combined with the final"] # [doc = " table lookup, which is good for performance."] # [derive (PartialEq , PartialOrd , Copy , Clone)] pub (super) struct Mdf (u32) ;
};
}
