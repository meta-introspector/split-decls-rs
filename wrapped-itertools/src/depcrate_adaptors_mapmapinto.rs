// Generated macro for MapInto (type)
macro_rules! Depcrate_adaptors_mapMapInto {
() => {
// Module: crate::adaptors::map
// Provides: {"MapInto"}
// Dependencies: {}
# [doc = " An iterator adapter to apply `Into` conversion to each element."] # [doc = ""] # [doc = " See [`.map_into()`](crate::Itertools::map_into) for more information."] pub type MapInto < I , R > = MapSpecialCase < I , MapSpecialCaseFnInto < R > > ;
};
}
