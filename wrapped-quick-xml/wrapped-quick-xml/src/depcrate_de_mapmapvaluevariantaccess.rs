// Generated macro for MapValueVariantAccess (struct)
macro_rules! Depcrate_de_mapMapValueVariantAccess {
() => {
// Module: crate::de::map
// Provides: {"MapValueVariantAccess"}
// Dependencies: {}
struct MapValueVariantAccess < 'de , 'd , 'm , R , E > where R : XmlRead < 'de > , E : EntityResolver , { # [doc = " Access to the map that created this enum accessor. Gives access to the"] # [doc = " context, such as list of fields, that current map known about."] map : & 'm mut ElementMapAccess < 'de , 'd , R , E > , # [doc = " `true` if variant should be deserialized from a textual content"] # [doc = " and `false` if from tag"] is_text : bool , }
};
}
