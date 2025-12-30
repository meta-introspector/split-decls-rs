// Generated macro for impl_866 (impl)
macro_rules! Depcrate_transliterate_providerimpl_866 {
() => {
// Module: crate::transliterate::provider
// Provides: {"impl_866"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for RuleBasedTransliterator < 'de > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { use serde :: de :: Error ; # [derive (serde :: Deserialize)] pub struct Raw < 'a > { pub visibility : bool , # [serde (borrow)] pub variable_table : VarTable < 'a > , # [serde (borrow)] pub filter : CodePointInversionList < 'a > , # [serde (borrow)] pub id_group_list : VarZeroVec < 'a , VarZeroSlice < SimpleIdULE > > , # [serde (borrow)] pub rule_group_list : VarZeroVec < 'a , VarZeroSlice < RuleULE , Index32 > , Index32 > , } let Raw { visibility , variable_table , filter , id_group_list , rule_group_list , } = Raw :: deserialize (deserializer) ? ; if id_group_list . len () != rule_group_list . len () { return Err (D :: Error :: custom ("invalid data: id_group_list and rule_group_list have different lengths" ,)) ; } Ok (Self { visibility , variable_table , filter , id_group_list , rule_group_list , }) } }
};
}
