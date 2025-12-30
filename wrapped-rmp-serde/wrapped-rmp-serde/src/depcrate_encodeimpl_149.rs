// Generated macro for impl_149 (impl)
macro_rules! Depcrate_encodeimpl_149 {
() => {
// Module: crate::encode
// Provides: {"impl_149"}
// Dependencies: {}
impl < W : Write > Serializer < W , DefaultConfig > { # [doc = " Constructs a new `MessagePack` serializer whose output will be written to the writer"] # [doc = " specified."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This is the default constructor, which returns a serializer that will serialize structs"] # [doc = " and enums using the most compact representation."] # [inline] pub fn new (wr : W) -> Self { Self { wr , depth : 1024 , config : RuntimeConfig :: new (DefaultConfig) , _back_compat_config : PhantomData , } } }
};
}
