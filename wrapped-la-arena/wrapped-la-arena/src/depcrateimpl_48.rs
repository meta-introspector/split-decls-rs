// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl < T > fmt :: Debug for IdxRange < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple (& format ! ("IdxRange::<{}>" , std :: any :: type_name ::< T > ())) . field (& self . range) . finish () } }
};
}
