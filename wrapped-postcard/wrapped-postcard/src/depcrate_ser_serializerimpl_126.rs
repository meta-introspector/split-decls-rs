// Generated macro for impl_126 (impl)
macro_rules! Depcrate_ser_serializerimpl_126 {
() => {
// Module: crate::ser::serializer
// Provides: {"impl_126"}
// Dependencies: {}
impl < F : Flavor > Serializer < F > { # [doc = " Attempt to push a variably encoded [usize] into the output data stream"] # [inline] pub (crate) fn try_push_varint_usize (& mut self , data : usize) -> Result < () > { let mut buf = [0u8 ; varint_max :: < usize > ()] ; let used_buf = varint_usize (data , & mut buf) ; self . output . try_extend (used_buf) } # [doc = " Attempt to push a variably encoded [u128] into the output data stream"] # [inline] pub (crate) fn try_push_varint_u128 (& mut self , data : u128) -> Result < () > { let mut buf = [0u8 ; varint_max :: < u128 > ()] ; let used_buf = varint_u128 (data , & mut buf) ; self . output . try_extend (used_buf) } # [doc = " Attempt to push a variably encoded [u64] into the output data stream"] # [inline] pub (crate) fn try_push_varint_u64 (& mut self , data : u64) -> Result < () > { let mut buf = [0u8 ; varint_max :: < u64 > ()] ; let used_buf = varint_u64 (data , & mut buf) ; self . output . try_extend (used_buf) } # [doc = " Attempt to push a variably encoded [u32] into the output data stream"] # [inline] pub (crate) fn try_push_varint_u32 (& mut self , data : u32) -> Result < () > { let mut buf = [0u8 ; varint_max :: < u32 > ()] ; let used_buf = varint_u32 (data , & mut buf) ; self . output . try_extend (used_buf) } # [doc = " Attempt to push a variably encoded [u16] into the output data stream"] # [inline] pub (crate) fn try_push_varint_u16 (& mut self , data : u16) -> Result < () > { let mut buf = [0u8 ; varint_max :: < u16 > ()] ; let used_buf = varint_u16 (data , & mut buf) ; self . output . try_extend (used_buf) } }
};
}
