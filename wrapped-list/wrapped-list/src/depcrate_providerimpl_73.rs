// Generated macro for impl_73 (impl)
macro_rules! Depcrate_providerimpl_73 {
() => {
// Module: crate::provider
// Provides: {"impl_73"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'data > SpecialCasePattern < 'data > { fn deserialize_option < 'de : 'data , D > (deserializer : D) -> Result < Option < Self > , D :: Error > where D : serde :: de :: Deserializer < 'de > , { use serde :: Deserialize ; # [derive (Deserialize)] struct SpecialCasePatternOptionalDfa < 'data > { # [cfg_attr (feature = "serde" , serde (borrow , deserialize_with = "SerdeDFA::maybe_deserialize"))] pub condition : Option < SerdeDFA < 'data > > , # [cfg_attr (feature = "serde" , serde (borrow))] pub pattern : ListJoinerPattern < 'data > , } Ok (match Option :: < SpecialCasePatternOptionalDfa < 'data > > :: deserialize (deserializer) ? { Some (SpecialCasePatternOptionalDfa { condition : Some (condition) , pattern , }) => Some (SpecialCasePattern { condition , pattern }) , _ => None , } ,) } }
};
}
