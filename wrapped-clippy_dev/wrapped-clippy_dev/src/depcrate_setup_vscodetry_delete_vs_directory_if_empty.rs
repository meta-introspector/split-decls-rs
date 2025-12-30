// Generated macro for try_delete_vs_directory_if_empty (function)
macro_rules! Depcrate_setup_vscodetry_delete_vs_directory_if_empty {
() => {
// Module: crate::setup::vscode
// Provides: {"try_delete_vs_directory_if_empty"}
// Dependencies: {}
# [doc = " This function will try to delete the `.vscode` directory if it's empty."] # [doc = " It may fail silently."] fn try_delete_vs_directory_if_empty () { let path = Path :: new (VSCODE_DIR) ; if path . read_dir () . is_ok_and (| mut iter | iter . next () . is_none ()) { let _silence_result = fs :: remove_dir (path) ; } else { } }
};
}
