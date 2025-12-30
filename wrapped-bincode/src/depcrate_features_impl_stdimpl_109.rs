// Generated macro for impl_109 (impl)
macro_rules! Depcrate_features_impl_stdimpl_109 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'a , W : std :: io :: Write > IoWriter < 'a , W > { pub fn new (writer : & 'a mut W) -> Self { Self { writer , bytes_written : 0 , } } pub const fn bytes_written (& self) -> usize { self . bytes_written } }
};
}
