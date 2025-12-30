// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl < T > Debug for Index < T > { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_tuple ("Index") . field (& self . index) . field (& self . generation) . finish () } }
};
}
