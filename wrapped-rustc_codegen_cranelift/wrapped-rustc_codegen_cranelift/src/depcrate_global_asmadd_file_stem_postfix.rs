// Generated macro for add_file_stem_postfix (function)
macro_rules! Depcrate_global_asmadd_file_stem_postfix {
() => {
// Module: crate::global_asm
// Provides: {"add_file_stem_postfix"}
// Dependencies: {}
pub (crate) fn add_file_stem_postfix (mut path : PathBuf , postfix : & str) -> PathBuf { let mut new_filename = path . file_stem () . unwrap () . to_owned () ; new_filename . push (postfix) ; if let Some (extension) = path . extension () { new_filename . push (".") ; new_filename . push (extension) ; } path . set_file_name (new_filename) ; path }
};
}
