// Generated macro for impl_166 (impl)
macro_rules! Depcrate_encodeimpl_166 {
() => {
// Module: crate::encode
// Provides: {"impl_166"}
// Dependencies: {}
impl < W , C : SerializerConfig > From < & Serializer < W , C > > for UnknownLengthCompound { fn from (se : & Serializer < W , C >) -> Self { Self { se : Serializer { wr : Vec :: with_capacity (128) , config : RuntimeConfig :: new (se . config) , depth : se . depth , _back_compat_config : PhantomData , } , elem_count : 0 , } } }
};
}
