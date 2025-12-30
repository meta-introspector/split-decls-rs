// Generated macro for impl_127 (impl)
macro_rules! Depcrateimpl_127 {
() => {
// Module: crate
// Provides: {"impl_127"}
// Dependencies: {}
impl Binary for FixedBitSet { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { if f . alternate () { f . write_str ("0b") ? ; } for i in 0 .. self . length { if self [i] { f . write_char ('1') ? ; } else { f . write_char ('0') ? ; } } Ok (()) } }
};
}
