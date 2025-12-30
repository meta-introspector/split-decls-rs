// Generated macro for impl_2008 (impl)
macro_rules! Depcrate_serializeimpl_2008 {
() => {
// Module: crate::serialize
// Provides: {"impl_2008"}
// Dependencies: {}
impl < 'a , DB > fmt :: Debug for Output < 'a , '_ , DB > where < DB :: BindCollector < 'a > as BindCollector < 'a , DB > > :: Buffer : fmt :: Debug , DB : Backend , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . out . fmt (f) } }
};
}
