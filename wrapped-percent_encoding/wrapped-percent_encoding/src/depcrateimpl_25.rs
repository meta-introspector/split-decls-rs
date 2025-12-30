// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl fmt :: Display for PercentEncode < '_ > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for c in (* self) . clone () { formatter . write_str (c) ? } Ok (()) } }
};
}
