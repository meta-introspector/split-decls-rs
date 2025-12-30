// Generated macro for duplicates (function)
macro_rules! Depcrate_duplicates_implduplicates {
() => {
// Module: crate::duplicates_impl
// Provides: {"duplicates"}
// Dependencies: {}
# [doc = " Create a new `Duplicates` iterator."] pub fn duplicates < I > (iter : I) -> Duplicates < I > where I : Iterator , I :: Item : Eq + Hash , { Duplicates :: new (iter , private :: ById) }
};
}
