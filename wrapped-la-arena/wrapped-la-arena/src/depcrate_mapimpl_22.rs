// Generated macro for impl_22 (impl)
macro_rules! Depcrate_mapimpl_22 {
() => {
// Module: crate::map
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a , IDX , V > VacantEntry < 'a , IDX , V > { # [doc = " Sets the value of the entry with the `VacantEntry`’s key, and returns a mutable reference to it."] pub fn insert (self , value : V) -> & 'a mut V { self . slot . insert (value) } }
};
}
