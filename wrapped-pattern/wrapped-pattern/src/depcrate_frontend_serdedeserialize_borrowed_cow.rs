// Generated macro for deserialize_borrowed_cow (function)
macro_rules! Depcrate_frontend_serdedeserialize_borrowed_cow {
() => {
// Module: crate::frontend::serde
// Provides: {"deserialize_borrowed_cow"}
// Dependencies: {}
# [doc (hidden)] pub fn deserialize_borrowed_cow < 'de , 'data , B , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Cow < 'data , Pattern < B > > , D :: Error > where 'de : 'data , B : PatternBackend < Store = str > , B :: PlaceholderKeyCow < 'data > : Deserialize < 'de > , & 'data B :: Store : Deserialize < 'de > , { if deserializer . is_human_readable () { Box :: < Pattern < B > > :: deserialize (deserializer) . map (Cow :: Owned) } else { < & Pattern < B > > :: deserialize (deserializer) . map (Cow :: Borrowed) } }
};
}
