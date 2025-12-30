// Generated macro for impl_186 (impl)
macro_rules! Depcrateimpl_186 {
() => {
// Module: crate
// Provides: {"impl_186"}
// Dependencies: {}
impl < T : Debug , const N : usize > Debug for SmallVec < T , N > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
