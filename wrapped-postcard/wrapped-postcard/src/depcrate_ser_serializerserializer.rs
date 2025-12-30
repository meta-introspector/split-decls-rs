// Generated macro for Serializer (struct)
macro_rules! Depcrate_ser_serializerSerializer {
() => {
// Module: crate::ser::serializer
// Provides: {"Serializer"}
// Dependencies: {}
# [doc = " A `serde` compatible serializer, generic over \"Flavors\" of serializing plugins."] # [doc = ""] # [doc = " It should rarely be necessary to directly use this type unless you are implementing your"] # [doc = " own [`SerFlavor`]."] # [doc = ""] # [doc = " See the docs for [`SerFlavor`] for more information about \"flavors\" of serialization"] # [doc = ""] # [doc = " [`SerFlavor`]: crate::ser_flavors::Flavor"] pub struct Serializer < F > where F : Flavor , { # [doc = " This is the Flavor(s) that will be used to modify or store any bytes generated"] # [doc = " by serialization"] pub output : F , }
};
}
