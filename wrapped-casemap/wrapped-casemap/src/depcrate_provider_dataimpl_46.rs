// Generated macro for impl_46 (impl)
macro_rules! Depcrate_provider_dataimpl_46 {
() => {
// Module: crate::provider::data
// Provides: {"impl_46"}
// Dependencies: {}
impl CaseType { pub (crate) const CASE_MASK : u16 = 0x3 ; # [inline] pub (crate) fn from_masked_bits (b : u16) -> Option < Self > { debug_assert ! (b & Self :: CASE_MASK == b) ; match b { 0 => None , 1 => Some (CaseType :: Lower) , 2 => Some (CaseType :: Upper) , _ => Some (CaseType :: Title) , } } }
};
}
