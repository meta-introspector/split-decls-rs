// Generated macro for EnumAccess (struct)
macro_rules! Depcrate_de_varEnumAccess {
() => {
// Module: crate::de::var
// Provides: {"EnumAccess"}
// Dependencies: {}
# [doc = " An enum access"] pub struct EnumAccess < 'de , 'd , R , E > where R : XmlRead < 'de > , E : EntityResolver , { de : & 'd mut Deserializer < 'de , R , E > , }
};
}
