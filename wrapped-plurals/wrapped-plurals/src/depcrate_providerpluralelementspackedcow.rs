// Generated macro for PluralElementsPackedCow (struct)
macro_rules! Depcrate_providerPluralElementsPackedCow {
() => {
// Module: crate::provider
// Provides: {"PluralElementsPackedCow"}
// Dependencies: {}
# [doc = " A sized packed [`PluralElements`] suitable for use in data structs."] # [doc = ""] # [doc = " This type has the following limitations:"] # [doc = ""] # [doc = " 1. It only supports `str`"] # [doc = " 2. It does not implement [`VarULE`] so it can't be used in a [`VarZeroSlice`]"] # [doc = " 3. It always serializes the [`FourBitMetadata`] as 0"] # [doc = ""] # [doc = " Use [`PluralElementsPackedULE`] directly if you need these additional features."] # [derive (Debug , PartialEq , Yokeable , ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_plurals :: provider))] # [cfg_attr (feature = "serde" , serde (transparent , bound (serialize = "V: serde::Serialize + PartialEq" , deserialize = "Box<PluralElementsPackedULE<V>>: serde::Deserialize<'de>")))] pub struct PluralElementsPackedCow < 'data , V : VarULE + ? Sized > { # [doc = " The encoded elements."] # [cfg_attr (feature = "serde" , serde (borrow , deserialize_with = "deserialize_plural_elements_packed_cow::<_, V>"))] pub elements : Cow < 'data , PluralElementsPackedULE < V > > , }
};
}
