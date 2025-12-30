// Generated macro for impl_94 (impl)
macro_rules! Depcrate_iterators_pairimpl_94 {
() => {
// Module: crate::iterators::pair
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'i , R : PartialEq > PartialEq for Pair < 'i , R > { fn eq (& self , other : & Pair < 'i , R >) -> bool { Rc :: ptr_eq (& self . queue , & other . queue) && ptr :: eq (self . input , other . input) && self . start == other . start } }
};
}
