// Generated macro for Serializer (struct)
macro_rules! Depcrate_export_serializers_postcardSerializer {
() => {
// Module: crate::export::serializers::postcard
// Provides: {"Serializer"}
// Dependencies: {}
# [doc = " A serializer for [Postcard](postcard)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu_provider_fs::export::serializers;"] # [doc = " use icu_provider_fs::export::FilesystemExporter;"] # [doc = ""] # [doc = " let serializer = serializers::Postcard;"] # [doc = ""] # [doc = " // Then pass it to a FilesystemExporter:"] # [doc = " let demo_path = std::env::temp_dir().join(\"icu4x_postcard_serializer_demo\");"] # [doc = " FilesystemExporter::try_new("] # [doc = "     Box::from(serializer),"] # [doc = "     demo_path.clone().into(),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = " # std::fs::remove_dir_all(&demo_path).expect(\"Cleaning up test directory\");"] # [doc = " ```"] # [allow (clippy :: exhaustive_structs)] # [derive (Debug , Default)] pub struct Serializer ;
};
}
