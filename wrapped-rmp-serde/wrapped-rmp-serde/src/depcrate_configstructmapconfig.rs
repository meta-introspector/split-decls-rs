// Generated macro for StructMapConfig (struct)
macro_rules! Depcrate_configStructMapConfig {
() => {
// Module: crate::config
// Provides: {"StructMapConfig"}
// Dependencies: {}
# [doc = " Config wrapper, that overrides struct serialization by packing as a map with field names."] # [doc = ""] # [doc = " MessagePack specification does not tell how to serialize structs. This trait allows you to"] # [doc = " extend serialization to match your app's requirements."] # [doc = ""] # [doc = " Default `Serializer` implementation writes structs as a tuple, i.e. only its length is encoded,"] # [doc = " because it is the most compact representation."] # [derive (Copy , Clone , Debug)] pub struct StructMapConfig < C > (C) ;
};
}
