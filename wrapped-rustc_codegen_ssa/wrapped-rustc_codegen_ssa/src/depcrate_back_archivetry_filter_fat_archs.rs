// Generated macro for try_filter_fat_archs (function)
macro_rules! Depcrate_back_archivetry_filter_fat_archs {
() => {
// Module: crate::back::archive
// Provides: {"try_filter_fat_archs"}
// Dependencies: {}
fn try_filter_fat_archs (archs : & [impl FatArch] , target_arch : object :: Architecture , archive_path : & Path , archive_map_data : & [u8] ,) -> io :: Result < Option < PathBuf > > { let desired = match archs . iter () . find (| a | a . architecture () == target_arch) { Some (a) => a , None => return Ok (None) , } ; let (mut new_f , extracted_path) = tempfile :: Builder :: new () . suffix (archive_path . file_name () . unwrap ()) . tempfile () ? . keep () . unwrap () ; new_f . write_all (desired . data (archive_map_data) . map_err (| e | io :: Error :: new (io :: ErrorKind :: Other , e)) ? ,) ? ; Ok (Some (extracted_path)) }
};
}
