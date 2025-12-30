// Generated macro for impl_188 (impl)
macro_rules! Depcrateimpl_188 {
() => {
// Module: crate
// Provides: {"impl_188"}
// Dependencies: {}
impl < T : Debug , const N : usize > Debug for Drain < '_ , T , N > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }
};
}
