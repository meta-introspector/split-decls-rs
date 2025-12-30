// Generated macro for map_ok (function)
macro_rules! Depcrate_adaptors_mapmap_ok {
() => {
// Module: crate::adaptors::map
// Provides: {"map_ok"}
// Dependencies: {}
# [doc = " Create a new `MapOk` iterator."] pub fn map_ok < I , F , T , U , E > (iter : I , f : F) -> MapOk < I , F > where I : Iterator < Item = Result < T , E > > , F : FnMut (T) -> U , { MapSpecialCase { iter , f : MapSpecialCaseFnOk (f) , } }
};
}
