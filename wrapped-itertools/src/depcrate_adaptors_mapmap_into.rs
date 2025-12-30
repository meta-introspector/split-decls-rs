// Generated macro for map_into (function)
macro_rules! Depcrate_adaptors_mapmap_into {
() => {
// Module: crate::adaptors::map
// Provides: {"map_into"}
// Dependencies: {}
# [doc = " Create a new [`MapInto`] iterator."] pub fn map_into < I , R > (iter : I) -> MapInto < I , R > { MapSpecialCase { iter , f : MapSpecialCaseFnInto (PhantomData) , } }
};
}
