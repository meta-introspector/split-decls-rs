// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a , T : ? Sized + FromHex > FromHex for & 'a T { fn from_hex < U : FromIterator < u8 > > (& self) -> Result < U , FromHexError > { (* * self) . from_hex () } }
};
}
