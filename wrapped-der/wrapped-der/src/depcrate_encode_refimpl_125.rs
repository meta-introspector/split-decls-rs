// Generated macro for impl_125 (impl)
macro_rules! Depcrate_encode_refimpl_125 {
() => {
// Module: crate::encode_ref
// Provides: {"impl_125"}
// Dependencies: {}
impl < T > Encode for EncodeRef < '_ , T > where T : Encode , { fn encoded_len (& self) -> Result < Length > { self . 0 . encoded_len () } fn encode (& self , writer : & mut impl Writer) -> Result < () > { self . 0 . encode (writer) } }
};
}
