// Generated macro for Trim (enum)
macro_rules! DepcrateTrim {
() => {
// Module: crate
// Provides: {"Trim"}
// Dependencies: {}
# [doc = " The whitespace preservation behaviour when reading CSV data."] # [derive (Clone , Copy , Debug , Default , PartialEq)] # [non_exhaustive] pub enum Trim { # [doc = " Preserves fields and headers. This is the default."] # [default] None , # [doc = " Trim whitespace from headers."] Headers , # [doc = " Trim whitespace from fields, but not headers."] Fields , # [doc = " Trim whitespace from fields and headers."] All , }
};
}
