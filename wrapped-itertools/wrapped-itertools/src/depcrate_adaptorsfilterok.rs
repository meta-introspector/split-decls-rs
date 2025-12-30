// Generated macro for FilterOk (struct)
macro_rules! Depcrate_adaptorsFilterOk {
() => {
// Module: crate::adaptors
// Provides: {"FilterOk"}
// Dependencies: {}
# [doc = " An iterator adapter to filter values within a nested `Result::Ok`."] # [doc = ""] # [doc = " See [`.filter_ok()`](crate::Itertools::filter_ok) for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct FilterOk < I , F > { iter : I , f : F , }
};
}
