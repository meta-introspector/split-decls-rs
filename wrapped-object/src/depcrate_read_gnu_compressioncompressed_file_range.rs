// Generated macro for compressed_file_range (function)
macro_rules! Depcrate_read_gnu_compressioncompressed_file_range {
() => {
// Module: crate::read::gnu_compression
// Provides: {"compressed_file_range"}
// Dependencies: {}
pub (super) fn compressed_file_range < 'data , R : ReadRef < 'data > > (file_data : R , section_offset : u64 , section_size : u64 ,) -> read :: Result < CompressedFileRange > { let mut offset = section_offset ; let header = file_data . read_bytes (& mut offset , 8) . read_error ("GNU compressed section is too short") ? ; if header != b"ZLIB\0\0\0\0" { return Err (Error ("Invalid GNU compressed section header")) ; } let uncompressed_size = file_data . read :: < U32Bytes < _ > > (& mut offset) . read_error ("GNU compressed section is too short") ? . get (endian :: BigEndian) . into () ; let compressed_size = section_size . checked_sub (offset - section_offset) . read_error ("GNU compressed section is too short") ? ; Ok (CompressedFileRange { format : CompressionFormat :: Zlib , offset , compressed_size , uncompressed_size , }) }
};
}
