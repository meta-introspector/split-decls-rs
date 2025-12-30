// Generated macro for Deserializer (struct)
macro_rules! Depcrate_de_deserializerDeserializer {
() => {
// Module: crate::de::deserializer
// Provides: {"Deserializer"}
// Dependencies: {}
# [doc = " A `serde` compatible deserializer, generic over “Flavors” of deserializing plugins."] # [doc = ""] # [doc = " Please note that postcard messages are not self-describing and therefore incompatible with"] # [doc = " [internally tagged enums](https://serde.rs/enum-representations.html#internally-tagged)."] pub struct Deserializer < 'de , F : Flavor < 'de > > { flavor : F , _plt : PhantomData < & 'de () > , }
};
}
