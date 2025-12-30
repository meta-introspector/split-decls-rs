// Generated macro for normalize_diag (function)
macro_rules! Depcratenormalize_diag {
() => {
// Module: crate
// Provides: {"normalize_diag"}
// Dependencies: {}
# [doc = " The target directory can sometimes be stored in the file name of spans."] # [doc = " This is problematic since the directory in constructed from the thread"] # [doc = " ID and also used in our CI to determine if two lint emissions are the"] # [doc = " same or not. This function simply normalizes the `_<thread_id>` to `_*`."] fn normalize_diag (mut message : cargo_metadata :: diagnostic :: Diagnostic , thread_target_dir : & str ,) -> cargo_metadata :: diagnostic :: Diagnostic { let mut dir_found = false ; message . spans . iter_mut () . filter (| span | span . file_name . starts_with (thread_target_dir)) . for_each (| span | { dir_found = true ; span . file_name . replace_range (0 .. thread_target_dir . len () , shared_target_dir ("_*") . to_str () . unwrap ()) ; }) ; if dir_found && let Some (rendered) = & mut message . rendered { * rendered = rendered . replace (thread_target_dir , shared_target_dir ("_*") . to_str () . unwrap ()) ; } message }
};
}
