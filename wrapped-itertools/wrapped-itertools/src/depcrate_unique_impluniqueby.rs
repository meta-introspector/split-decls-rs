// Generated macro for UniqueBy (struct)
macro_rules! Depcrate_unique_implUniqueBy {
() => {
// Module: crate::unique_impl
// Provides: {"UniqueBy"}
// Dependencies: {}
# [doc = " An iterator adapter to filter out duplicate elements."] # [doc = ""] # [doc = " See [`.unique_by()`](crate::Itertools::unique) for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct UniqueBy < I : Iterator , V , F > { iter : I , used : HashMap < V , () > , f : F , }
};
}
