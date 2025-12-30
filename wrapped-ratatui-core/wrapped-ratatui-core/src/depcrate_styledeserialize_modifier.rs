// Generated macro for deserialize_modifier (function)
macro_rules! Depcrate_styledeserialize_modifier {
() => {
// Module: crate::style
// Provides: {"deserialize_modifier"}
// Dependencies: {}
# [cfg (feature = "serde")] # [doc = " Deserialize a [`Modifier`] while treating missing or `null` values as empty."] # [doc = ""] # [doc = " This helper is used with serde to coerce absent or `null` modifier fields to"] # [doc = " [`Modifier::empty`], allowing configuration files to omit these fields"] # [doc = " without triggering deserialization errors."] fn deserialize_modifier < 'de , D > (deserializer : D) -> Result < Modifier , D :: Error > where D : serde :: Deserializer < 'de > , { use serde :: Deserialize ; Option :: < Modifier > :: deserialize (deserializer) . map (| modifier | modifier . unwrap_or_else (Modifier :: empty)) }
};
}
