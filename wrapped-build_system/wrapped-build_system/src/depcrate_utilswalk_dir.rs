// Generated macro for walk_dir (function)
macro_rules! Depcrate_utilswalk_dir {
() => {
// Module: crate::utils
// Provides: {"walk_dir"}
// Dependencies: {}
pub fn walk_dir < P , D , F > (dir : P , dir_cb : & mut D , file_cb : & mut F , recursive : bool ,) -> Result < () , String > where P : AsRef < Path > , D : FnMut (& Path) -> Result < () , String > , F : FnMut (& Path) -> Result < () , String > , { let dir = dir . as_ref () ; for entry in fs :: read_dir (dir) . map_err (| error | format ! ("Failed to read dir `{}`: {:?}" , dir . display () , error)) ? { let entry = entry . map_err (| error | format ! ("Failed to read entry in `{}`: {:?}" , dir . display () , error)) ? ; let entry_path = entry . path () ; if entry_path . is_dir () { dir_cb (& entry_path) ? ; if recursive { walk_dir (entry_path , dir_cb , file_cb , recursive) ? ; } } else { file_cb (& entry_path) ? ; } } Ok (()) }
};
}
