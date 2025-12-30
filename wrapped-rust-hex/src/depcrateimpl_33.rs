// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl < T : AsRef < [u8] > + ? Sized > ToHex for T { fn encode_hex < U : iter :: FromIterator < char > > (& self) -> U { encode_to_iter (HEX_CHARS_LOWER , self . as_ref ()) } fn encode_hex_upper < U : iter :: FromIterator < char > > (& self) -> U { encode_to_iter (HEX_CHARS_UPPER , self . as_ref ()) } }
};
}
