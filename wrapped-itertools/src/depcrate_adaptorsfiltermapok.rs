// Generated macro for FilterMapOk (struct)
macro_rules! Depcrate_adaptorsFilterMapOk {
() => {
// Module: crate::adaptors
// Provides: {"FilterMapOk"}
// Dependencies: {}
# [doc = " An iterator adapter to filter and apply a transformation on values within a nested `Result::Ok`."] # [doc = ""] # [doc = " See [`.filter_map_ok()`](crate::Itertools::filter_map_ok) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct FilterMapOk < I , F > { iter : I , f : F , }
};
}
