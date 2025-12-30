// Generated macro for VariantAccess (struct)
macro_rules! Depcrate_de_varVariantAccess {
() => {
// Module: crate::de::var
// Provides: {"VariantAccess"}
// Dependencies: {}
pub struct VariantAccess < 'de , 'd , R , E > where R : XmlRead < 'de > , E : EntityResolver , { de : & 'd mut Deserializer < 'de , R , E > , # [doc = " `true` if variant should be deserialized from a textual content"] # [doc = " and `false` if from tag"] is_text : bool , }
};
}
