// Generated macro for DuplicatesBy (type)
macro_rules! Depcrate_duplicates_implDuplicatesBy {
() => {
// Module: crate::duplicates_impl
// Provides: {"DuplicatesBy"}
// Dependencies: {}
# [doc = " An iterator adapter to filter for duplicate elements."] # [doc = ""] # [doc = " See [`.duplicates_by()`](crate::Itertools::duplicates_by) for more information."] pub type DuplicatesBy < I , V , F > = private :: DuplicatesBy < I , V , private :: ByFn < F > > ;
};
}
