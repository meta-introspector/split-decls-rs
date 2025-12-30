// Generated macro for mmap (module)
macro_rules! Depcratemmap {
() => {
// Module: crate
// Provides: {"mmap"}
// Dependencies: {}
mod mmap { use std :: path :: Path ; pub fn read_only (path : & Path) -> std :: io :: Result < memmap2 :: Mmap > { let file = std :: fs :: File :: open (path) ? ; # [allow (unsafe_code)] unsafe { memmap2 :: MmapOptions :: new () . map_copy_read_only (& file) } } }
};
}
