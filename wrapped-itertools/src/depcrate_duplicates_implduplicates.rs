// Generated macro for Duplicates (type)
macro_rules! Depcrate_duplicates_implDuplicates {
() => {
// Module: crate::duplicates_impl
// Provides: {"Duplicates"}
// Dependencies: {}
# [doc = " An iterator adapter to filter out duplicate elements."] # [doc = ""] # [doc = " See [`.duplicates()`](crate::Itertools::duplicates) for more information."] pub type Duplicates < I > = private :: DuplicatesBy < I , < I as Iterator > :: Item , private :: ById > ;
};
}
