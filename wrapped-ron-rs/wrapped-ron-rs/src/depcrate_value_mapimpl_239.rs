// Generated macro for impl_239 (impl)
macro_rules! Depcrate_value_mapimpl_239 {
() => {
// Module: crate::value::map
// Provides: {"impl_239"}
// Dependencies: {}
impl Hash for Map { fn hash < H : Hasher > (& self , state : & mut H) { self . iter () . for_each (| x | x . hash (state)) ; } }
};
}
