// Generated macro for impl_52 (impl)
macro_rules! Depcrate_deimpl_52 {
() => {
// Module: crate::de
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'de , R , T > StreamDeserializer < 'de , R , T > where R : Read < 'de > , T : de :: Deserialize < 'de > , { # [doc = " Create a new CBOR stream deserializer from one of the possible"] # [doc = " serde_cbor input sources."] # [doc = ""] # [doc = " Typically it is more convenient to use one of these methods instead:"] # [doc = ""] # [doc = " * `Deserializer::from_slice(...).into_iter()`"] # [doc = " * `Deserializer::from_reader(...).into_iter()`"] pub fn new (read : R) -> StreamDeserializer < 'de , R , T > { StreamDeserializer { de : Deserializer :: new (read) , output : PhantomData , lifetime : PhantomData , } } }
};
}
