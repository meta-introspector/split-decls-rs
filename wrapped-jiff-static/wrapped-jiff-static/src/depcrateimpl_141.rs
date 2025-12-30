// Generated macro for impl_141 (impl)
macro_rules! Depcrateimpl_141 {
() => {
// Module: crate
// Provides: {"impl_141"}
// Dependencies: {}
impl TzifLocalTimeType { fn quote (& self) -> proc_macro2 :: TokenStream { let TzifLocalTimeType { offset , is_dst , ref designation , ref indicator , } = * self ; let desig_start = designation . 0 ; let desig_end = designation . 1 ; let indicator = indicator . quote () ; quote ! { jiff :: shared :: TzifLocalTimeType { offset : # offset , is_dst : # is_dst , designation : (# desig_start , # desig_end) , indicator : # indicator , } } } }
};
}
