// Generated macro for MapOk (type)
macro_rules! Depcrate_adaptors_mapMapOk {
() => {
// Module: crate::adaptors::map
// Provides: {"MapOk"}
// Dependencies: {}
# [doc = " An iterator adapter to apply a transformation within a nested `Result::Ok`."] # [doc = ""] # [doc = " See [`.map_ok()`](crate::Itertools::map_ok) for more information."] pub type MapOk < I , F > = MapSpecialCase < I , MapSpecialCaseFnOk < F > > ;
};
}
