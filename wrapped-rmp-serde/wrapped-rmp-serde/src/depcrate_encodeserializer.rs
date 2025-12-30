// Generated macro for Serializer (struct)
macro_rules! Depcrate_encodeSerializer {
() => {
// Module: crate::encode
// Provides: {"Serializer"}
// Dependencies: {}
# [doc = " Represents MessagePack serialization implementation."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " MessagePack has no specification about how to encode enum types. Thus we are free to do"] # [doc = " whatever we want, so the given choice may be not ideal for you."] # [doc = ""] # [doc = " An enum value is represented as a single-entry map whose key is the variant"] # [doc = " id and whose value is a sequence containing all associated data. If the enum"] # [doc = " does not have associated data, the sequence is empty."] # [doc = ""] # [doc = " All instances of `ErrorKind::Interrupted` are handled by this function and the underlying"] # [doc = " operation is retried."] # [derive (Debug)] pub struct Serializer < W , C = DefaultConfig > { wr : W , depth : u16 , config : RuntimeConfig , _back_compat_config : PhantomData < C > , }
};
}
