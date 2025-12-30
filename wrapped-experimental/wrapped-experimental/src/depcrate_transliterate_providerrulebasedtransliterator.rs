// Generated macro for RuleBasedTransliterator (struct)
macro_rules! Depcrate_transliterate_providerRuleBasedTransliterator {
() => {
// Module: crate::transliterate::provider
// Provides: {"RuleBasedTransliterator"}
// Dependencies: {}
# [doc = " The data struct representing [UTS #35 transform rules](https://unicode.org/reports/tr35/tr35-general.html#Transforms)."] # [derive (Debug , Clone , PartialEq , Eq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_experimental :: transliterate :: provider))] pub struct RuleBasedTransliterator < 'a > { # [doc = " Whether this transliterator is accessible directly through the constructor."] # [doc = " Hidden transliterators are intended as dependencies for visible transliterators,"] # [doc = " see, e.g., [Devanagari-Latin](https://github.com/unicode-org/cldr/blob/main/common/transforms/Devanagari-Latin.xml)"] pub visibility : bool , # [doc = " The [`VarTable`] containing any special matchers (variables, UnicodeSets, ...) used by this transliterator."] pub variable_table : VarTable < 'a > , # [doc = " The filter for this transliterator. If there is none, the set of all code points is used."] pub filter : CodePointInversionList < 'a > , # [doc = " The list of transform rule groups this transliterator uses."] pub id_group_list : VarZeroVec < 'a , VarZeroSlice < SimpleIdULE > > , # [doc = " The list of conversion rule groups this transliterator uses."] pub rule_group_list : VarZeroVec < 'a , VarZeroSlice < RuleULE , Index32 > , Index32 > , }
};
}
