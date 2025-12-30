// Generated macro for impl_29 (impl)
macro_rules! Depcrate_features_borshimpl_29 {
() => {
// Module: crate::features::borsh
// Provides: {"impl_29"}
// Dependencies: {}
impl BorshSerialize for CompactString { fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . as_str () . serialize (writer) } }
};
}
