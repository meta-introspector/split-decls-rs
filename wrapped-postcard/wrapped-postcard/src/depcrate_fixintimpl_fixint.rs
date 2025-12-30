// Generated macro for impl_fixint (macro)
macro_rules! Depcrate_fixintimpl_fixint {
() => {
// Module: crate::fixint
// Provides: {"impl_fixint"}
// Dependencies: {}
macro_rules ! impl_fixint { ($ ($ int : ty) ,*) => { $ (impl Serialize for LE <$ int > { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . to_le_bytes () . serialize (serializer) } } impl <'de > Deserialize <'de > for LE <$ int > { # [inline] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer <'de >, { < _ as Deserialize >:: deserialize (deserializer) . map (<$ int >:: from_le_bytes) . map (Self) } } impl Serialize for BE <$ int > { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . to_be_bytes () . serialize (serializer) } } impl <'de > Deserialize <'de > for BE <$ int > { # [inline] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer <'de >, { < _ as Deserialize >:: deserialize (deserializer) . map (<$ int >:: from_be_bytes) . map (Self) } }) * } ; }
};
}
