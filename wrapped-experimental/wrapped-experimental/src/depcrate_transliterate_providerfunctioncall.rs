// Generated macro for FunctionCall (struct)
macro_rules! Depcrate_transliterate_providerFunctionCall {
() => {
// Module: crate::transliterate::provider
// Provides: {"FunctionCall"}
// Dependencies: {}
# [doc = " An inline recursive call to a transliterator with an arbitrary argument."] # [derive (Debug , Clone)] # [make_varule (FunctionCallULE)] # [zerovec :: skip_derive (Ord)] # [zerovec :: derive (Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize) , zerovec :: derive (Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize) , zerovec :: derive (Serialize))] pub struct FunctionCall < 'a > { # [doc = " The transliterator that will be called."] # [zerovec :: varule (SimpleIdULE)] # [cfg_attr (feature = "serde" , serde (borrow))] pub translit : SimpleId < 'a > , # [cfg_attr (feature = "serde" , serde (borrow))] # [doc = " The argument to be transliterated given to the transliterator."] pub arg : Cow < 'a , str > , }
};
}
