// Generated macro for deserialize_plural_elements_packed_cow (function)
macro_rules! Depcrate_providerdeserialize_plural_elements_packed_cow {
() => {
// Module: crate::provider
// Provides: {"deserialize_plural_elements_packed_cow"}
// Dependencies: {}
# [doc = " Helper function to properly deserialize a `Cow<PluralElementsPackedULE<V>>`"] # [doc = ""] # [doc = " Due to <https://github.com/rust-lang/rust/issues/130180>, you may need to qualify"] # [doc = " `V` when invoking this, like so:"] # [doc = ""] # [doc = " `#[serde(deserialize_with = \"deserialize_plural_elements_packed_cow::<_, str>\")]`"] # [doc = ""] # [doc = " See <https://github.com/unicode-org/icu4x/pull/1556>"] # [cfg (feature = "serde")] fn deserialize_plural_elements_packed_cow < 'de , 'data , D , V > (deserializer : D ,) -> Result < Cow < 'data , PluralElementsPackedULE < V > > , D :: Error > where 'de : 'data , D : serde :: Deserializer < 'de > , V : VarULE + ? Sized , Box < PluralElementsPackedULE < V > > : serde :: Deserialize < 'de > , { use serde :: Deserialize ; if deserializer . is_human_readable () { let value = Box :: < PluralElementsPackedULE < V > > :: deserialize (deserializer) ? ; Ok (Cow :: Owned (value)) } else { let value = < & 'de PluralElementsPackedULE < V > > :: deserialize (deserializer) ? ; Ok (Cow :: Borrowed (value)) } }
};
}
