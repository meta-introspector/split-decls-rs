// Generated macro for sealed (module)
macro_rules! Depcrate_configsealed {
() => {
// Module: crate::config
// Provides: {"sealed"}
// Dependencies: {}
pub (crate) mod sealed { use crate :: config :: BytesMode ; # [doc = " This is the inner trait - the real `SerializerConfig`."] # [doc = ""] # [doc = " This hack disallows external implementations and usage of `SerializerConfig` and thus"] # [doc = " allows us to change `SerializerConfig` methods freely without breaking backwards compatibility."] pub trait SerializerConfig : Copy { # [doc = " Determines the value of `Serializer::is_human_readable` and"] # [doc = " `Deserializer::is_human_readable`."] fn is_human_readable (& self) -> bool ; # [doc = " String struct fields"] fn is_named (& self) -> bool ; fn bytes (& self) -> BytesMode ; } }
};
}
