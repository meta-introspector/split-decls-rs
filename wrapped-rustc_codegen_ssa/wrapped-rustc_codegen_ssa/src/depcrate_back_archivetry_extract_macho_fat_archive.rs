// Generated macro for try_extract_macho_fat_archive (function)
macro_rules! Depcrate_back_archivetry_extract_macho_fat_archive {
() => {
// Module: crate::back::archive
// Provides: {"try_extract_macho_fat_archive"}
// Dependencies: {}
pub fn try_extract_macho_fat_archive (sess : & Session , archive_path : & Path ,) -> io :: Result < Option < PathBuf > > { let archive_map = unsafe { Mmap :: map (File :: open (& archive_path) ?) ? } ; let target_arch = match sess . target . arch . as_ref () { "aarch64" => object :: Architecture :: Aarch64 , "x86_64" => object :: Architecture :: X86_64 , _ => return Ok (None) , } ; if let Ok (h) = object :: read :: macho :: MachOFatFile32 :: parse (& * archive_map) { let archs = h . arches () ; try_filter_fat_archs (archs , target_arch , archive_path , & * archive_map) } else if let Ok (h) = object :: read :: macho :: MachOFatFile64 :: parse (& * archive_map) { let archs = h . arches () ; try_filter_fat_archs (archs , target_arch , archive_path , & * archive_map) } else { Ok (None) } }
};
}
