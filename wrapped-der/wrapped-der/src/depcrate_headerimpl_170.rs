// Generated macro for impl_170 (impl)
macro_rules! Depcrate_headerimpl_170 {
() => {
// Module: crate::header
// Provides: {"impl_170"}
// Dependencies: {}
impl Encode for Header { fn encoded_len (& self) -> Result < Length > { self . tag . encoded_len () ? + self . length . encoded_len () ? } fn encode (& self , writer : & mut impl Writer) -> Result < () > { self . tag . encode (writer) ? ; self . length . encode (writer) } }
};
}
