// Generated macro for ReadCacheOps (trait)
macro_rules! Depcrate_read_read_cacheReadCacheOps {
() => {
// Module: crate::read::read_cache
// Provides: {"ReadCacheOps"}
// Dependencies: {}
# [doc = " Operations required to implement [`ReadCache`]."] # [doc = ""] # [doc = " This is a subset of the `Read` and `Seek` traits."] # [doc = " A blanket implementation is provided for all types that implement"] # [doc = " `Read + Seek`."] # [allow (clippy :: len_without_is_empty)] pub trait ReadCacheOps { # [doc = " Return the length of the stream."] # [doc = ""] # [doc = " Equivalent to `std::io::Seek::seek(SeekFrom::End(0))`."] fn len (& mut self) -> Result < u64 , () > ; # [doc = " Seek to the given position in the stream."] # [doc = ""] # [doc = " Equivalent to `std::io::Seek::seek` with `SeekFrom::Start(pos)`."] fn seek (& mut self , pos : u64) -> Result < u64 , () > ; # [doc = " Read up to `buf.len()` bytes into `buf`."] # [doc = ""] # [doc = " Equivalent to `std::io::Read::read`."] fn read (& mut self , buf : & mut [u8]) -> Result < usize , () > ; # [doc = " Read exactly `buf.len()` bytes into `buf`."] # [doc = ""] # [doc = " Equivalent to `std::io::Read::read_exact`."] fn read_exact (& mut self , buf : & mut [u8]) -> Result < () , () > ; }
};
}
