// Generated macro for unique_path_in_tree (function)
macro_rules! Depcrate_tree_utilsunique_path_in_tree {
() => {
// Module: crate::tree::utils
// Provides: {"unique_path_in_tree"}
// Dependencies: {}
# [doc = " Produce a unique path within the directory that contains the file at `file_path` like `a/b`, using `editor`"] # [doc = " and `tree` to assure unique names, to obtain the tree at `a/` and `side_name` to more clearly signal"] # [doc = " where the file is coming from."] pub fn unique_path_in_tree (file_path : & BStr , editor : & tree :: Editor < '_ > , tree : & TreeNodes , side_name : & BStr ,) -> Result < BString , Error > { let mut buf = file_path . to_owned () ; buf . push (b'~') ; buf . extend (side_name . as_bytes () . iter () . copied () . map (| b | if b == b'/' { b'_' } else { b }) ,) ; let base_len = buf . len () ; let mut suffix = 0 ; while editor . get (to_components_bstring_ref (& buf)) . is_some () || tree . check_conflict (buf . as_bstr ()) . is_some () { buf . truncate (base_len) ; buf . push_str (format ! ("_{suffix}")) ; suffix += 1 ; } Ok (buf) }
};
}
