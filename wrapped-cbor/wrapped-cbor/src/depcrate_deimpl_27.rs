// Generated macro for impl_27 (impl)
macro_rules! Depcrate_deimpl_27 {
() => {
// Module: crate::de
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a , 'b > Deserializer < SliceReadFixed < 'a , 'b > > { # [doc (hidden)] pub fn from_slice_with_scratch (bytes : & 'a [u8] , scratch : & 'b mut [u8] ,) -> Deserializer < SliceReadFixed < 'a , 'b > > { Deserializer :: new (SliceReadFixed :: new (bytes , scratch)) } }
};
}
