// Generated macro for impl_62 (impl)
macro_rules! Depcrate_versionimpl_62 {
() => {
// Module: crate::version
// Provides: {"impl_62"}
// Dependencies: {}
impl Encode for Version { fn encoded_len (& self) -> der :: Result < der :: Length > { der :: Length :: from (1u8) . for_tlv (self . tag ()) } fn encode (& self , writer : & mut impl Writer) -> der :: Result < () > { u8 :: from (* self) . encode (writer) } }
};
}
