// Generated macro for Deserializer (struct)
macro_rules! Depcrate_decodeDeserializer {
() => {
// Module: crate::decode
// Provides: {"Deserializer"}
// Dependencies: {}
# [doc = " A Deserializer that reads bytes from a buffer."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " All instances of `ErrorKind::Interrupted` are handled by this function and the underlying"] # [doc = " operation is retried."] # [derive (Debug)] pub struct Deserializer < R , C = DefaultConfig > { rd : R , _config : PhantomData < C > , is_human_readable : bool , marker : Option < Marker > , depth : u16 , }
};
}
