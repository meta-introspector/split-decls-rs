// Generated macro for impl_77 (impl)
macro_rules! Depcrate_decodeimpl_77 {
() => {
// Module: crate::decode
// Provides: {"impl_77"}
// Dependencies: {}
impl < R : Read , C : SerializerConfig > Deserializer < R , C > { # [doc = " Consumes this deserializer and returns a new one, which will deserialize types with"] # [doc = " human-readable representations (`Deserializer::is_human_readable` will return `true`)."] # [doc = ""] # [doc = " This is primarily useful if you need to interoperate with serializations produced by older"] # [doc = " versions of `rmp-serde`."] # [inline] pub fn with_human_readable (self) -> Deserializer < R , HumanReadableConfig < C > > { let Self { rd , _config : _ , is_human_readable : _ , marker , depth } = self ; Deserializer { rd , is_human_readable : true , _config : PhantomData , marker , depth , } } # [doc = " Consumes this deserializer and returns a new one, which will deserialize types with"] # [doc = " binary representations (`Deserializer::is_human_readable` will return `false`)."] # [doc = ""] # [doc = " This is the default MessagePack deserialization mechanism, consuming the most compact"] # [doc = " representation."] # [inline] pub fn with_binary (self) -> Deserializer < R , BinaryConfig < C > > { let Self { rd , _config : _ , is_human_readable : _ , marker , depth } = self ; Deserializer { rd , is_human_readable : false , _config : PhantomData , marker , depth , } } }
};
}
