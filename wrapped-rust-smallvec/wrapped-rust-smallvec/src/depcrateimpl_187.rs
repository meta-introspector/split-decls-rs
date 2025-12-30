// Generated macro for impl_187 (impl)
macro_rules! Depcrateimpl_187 {
() => {
// Module: crate
// Provides: {"impl_187"}
// Dependencies: {}
impl < T : Debug , const N : usize > Debug for IntoIter < T , N > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . as_slice ()) . finish () } }
};
}
