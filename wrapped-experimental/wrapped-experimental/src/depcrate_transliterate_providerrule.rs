// Generated macro for Rule (struct)
macro_rules! Depcrate_transliterate_providerRule {
() => {
// Module: crate::transliterate::provider
// Provides: {"Rule"}
// Dependencies: {}
# [doc = " A conversion rule. The source patterns as well as the replacer use inlined private use characters"] # [doc = " that refer to elements of the [`VarTable`] for special matchers (variables, UnicodeSets, ...)."] # [derive (Debug , Clone)] # [make_varule (RuleULE)] # [zerovec :: skip_derive (Ord)] # [zerovec :: derive (Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize) , zerovec :: derive (Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize) , zerovec :: derive (Serialize))] pub struct Rule < 'a > { # [doc = " The pattern for the ante context. This is not replaced."] # [cfg_attr (feature = "serde" , serde (borrow))] pub ante : Cow < 'a , str > , # [doc = " The pattern for the key. This is what gets replaced."] # [cfg_attr (feature = "serde" , serde (borrow))] pub key : Cow < 'a , str > , # [doc = " The pattern for the post context. This is not replaced."] # [cfg_attr (feature = "serde" , serde (borrow))] pub post : Cow < 'a , str > , # [doc = " The replacer. The key gets replaced with this."] # [cfg_attr (feature = "serde" , serde (borrow))] pub replacer : Cow < 'a , str > , }
};
}
