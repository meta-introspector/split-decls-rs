// Generated macro for impl_136 (impl)
macro_rules! Depcrate_mapimpl_136 {
() => {
// Module: crate::map
// Provides: {"impl_136"}
// Dependencies: {}
impl FromIterator < (String , Value) > for Map < String , Value > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = (String , Value) > , { Map { map : FromIterator :: from_iter (iter) , } } }
};
}
