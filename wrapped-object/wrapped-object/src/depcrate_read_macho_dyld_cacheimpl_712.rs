// Generated macro for impl_712 (impl)
macro_rules! Depcrate_read_macho_dyld_cacheimpl_712 {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"impl_712"}
// Dependencies: {}
impl < E : Endian > macho :: DyldCacheImageInfo < E > { # [doc = " The file system path of this image."] # [doc = ""] # [doc = " `data` should be the main cache file, not the subcache containing the image."] pub fn path < 'data , R : ReadRef < 'data > > (& self , endian : E , data : R) -> Result < & 'data [u8] > { let r_start = self . path_file_offset . get (endian) . into () ; let r_end = data . len () . read_error ("Couldn't get data len()") ? ; data . read_bytes_at_until (r_start .. r_end , 0) . read_error ("Couldn't read dyld cache image path") } }
};
}
