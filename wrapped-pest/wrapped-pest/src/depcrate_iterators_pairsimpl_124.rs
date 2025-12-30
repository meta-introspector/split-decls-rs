// Generated macro for impl_124 (impl)
macro_rules! Depcrate_iterators_pairsimpl_124 {
() => {
// Module: crate::iterators::pairs
// Provides: {"impl_124"}
// Dependencies: {}
impl < 'i , R : PartialEq > PartialEq for Pairs < 'i , R > { fn eq (& self , other : & Pairs < 'i , R >) -> bool { Rc :: ptr_eq (& self . queue , & other . queue) && ptr :: eq (self . input , other . input) && self . start == other . start && self . end == other . end } }
};
}
