// Generated macro for SimpleId (struct)
macro_rules! Depcrate_transliterate_providerSimpleId {
() => {
// Module: crate::transliterate::provider
// Provides: {"SimpleId"}
// Dependencies: {}
# [doc = " The ID of a transliterator plus an optional filter."] # [derive (Debug , Clone)] # [make_varule (SimpleIdULE)] # [zerovec :: skip_derive (Ord)] # [zerovec :: derive (Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize) , zerovec :: derive (Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize) , zerovec :: derive (Serialize))] pub struct SimpleId < 'a > { # [doc = " The filter for the transliterator. If there is none, the set of all code points is used."] # [zerovec :: varule (CodePointInversionListULE)] # [cfg_attr (feature = "serde" , serde (borrow))] pub filter : CodePointInversionList < 'a > , # [doc = " The ID of the transliterator."] # [cfg_attr (feature = "serde" , serde (borrow))] pub id : Cow < 'a , str > , }
};
}
