// Generated macro for impl_97 (impl)
macro_rules! Depcrate_versionimpl_97 {
() => {
// Module: crate::version
// Provides: {"impl_97"}
// Dependencies: {}
impl Encode for Version { fn encoded_len (& self) -> der :: Result < der :: Length > { der :: Length :: ONE . for_tlv (self . tag ()) } fn encode (& self , writer : & mut impl Writer) -> der :: Result < () > { u8 :: from (* self) . encode (writer) } }
};
}
