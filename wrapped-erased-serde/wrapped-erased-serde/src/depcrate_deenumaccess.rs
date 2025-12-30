// Generated macro for EnumAccess (trait)
macro_rules! Depcrate_deEnumAccess {
() => {
// Module: crate::de
// Provides: {"EnumAccess"}
// Dependencies: {}
pub trait EnumAccess < 'de > { fn erased_variant_seed (& mut self , seed : & mut dyn DeserializeSeed < 'de > ,) -> Result < (Out , Variant < 'de >) , Error > ; }
};
}
