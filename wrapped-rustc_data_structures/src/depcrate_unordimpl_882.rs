// Generated macro for impl_882 (impl)
macro_rules! Depcrate_unordimpl_882 {
() => {
// Module: crate::unord
// Provides: {"impl_882"}
// Dependencies: {}
impl < T , I : Iterator < Item = T > > From < UnordItems < T , I > > for UnordBag < T > { fn from (value : UnordItems < T , I >) -> Self { UnordBag { inner : Vec :: from_iter (value . 0) } } }
};
}
