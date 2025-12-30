// Generated macro for Variant (struct)
macro_rules! Depcrate_deVariant {
() => {
// Module: crate::de
// Provides: {"Variant"}
// Dependencies: {}
pub struct Variant < 'de > { data : Any , unit_variant : unsafe fn (Any) -> Result < () , Error > , visit_newtype : unsafe fn (Any , seed : & mut dyn DeserializeSeed < 'de >) -> Result < Out , Error > , tuple_variant : unsafe fn (Any , len : usize , visitor : & mut dyn Visitor < 'de >) -> Result < Out , Error > , struct_variant : unsafe fn (Any , fields : & 'static [& 'static str] , visitor : & mut dyn Visitor < 'de > ,) -> Result < Out , Error > , }
};
}
