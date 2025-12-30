// Generated macro for impl_64 (impl)
macro_rules! Depcrate_iterators_flat_pairsimpl_64 {
() => {
// Module: crate::iterators::flat_pairs
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'i , R : Clone > Clone for FlatPairs < 'i , R > { fn clone (& self) -> FlatPairs < 'i , R > { FlatPairs { queue : Rc :: clone (& self . queue) , input : self . input , line_index : Rc :: clone (& self . line_index) , start : self . start , end : self . end , } } }
};
}
