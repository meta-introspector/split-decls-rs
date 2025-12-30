// Generated macro for RmpWrite (trait)
macro_rules! Depcrate_encodeRmpWrite {
() => {
// Module: crate::encode
// Provides: {"RmpWrite"}
// Dependencies: {}
# [doc = " A type that `rmp` supports writing into."] # [doc = ""] # [doc = " The methods of this trait should be considered an implementation detail (for now)."] # [doc = " It is currently sealed (can not be implemented by the user)."] # [doc = ""] # [doc = " See also [`std::io::Write`] and [`byteorder::WriteBytesExt`]"] # [doc = ""] # [doc = " Its primary implementations are [`std::io::Write`] and [`ByteBuf`]."] pub trait RmpWrite : sealed :: Sealed { type Error : RmpWriteErr ; # [doc = " Write a single byte to this stream"] # [inline] fn write_u8 (& mut self , val : u8) -> Result < () , Self :: Error > { let buf = [val] ; self . write_bytes (& buf) } # [doc = " Write a slice of bytes to the underlying stream"] # [doc = ""] # [doc = " This will either write all the bytes or return an error."] # [doc = " See also [`std::io::Write::write_all`]"] fn write_bytes (& mut self , buf : & [u8]) -> Result < () , Self :: Error > ; # [doc = " Write a single (signed) byte to this stream."] # [inline] # [doc (hidden)] fn write_data_u8 (& mut self , val : u8) -> Result < () , DataWriteError < Self :: Error > > { self . write_u8 (val) . map_err (DataWriteError) } # [doc = " Write a single (signed) byte to this stream."] # [inline] # [doc (hidden)] fn write_data_i8 (& mut self , val : i8) -> Result < () , DataWriteError < Self :: Error > > { self . write_data_u8 (val as u8) } write_byteorder_utils ! (write_data_u16 => u16 , write_data_u32 => u32 , write_data_u64 => u64 , write_data_i16 => i16 , write_data_i32 => i32 , write_data_i64 => i64 , write_data_f32 => f32 , write_data_f64 => f64) ; }
};
}
