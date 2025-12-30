// Generated macro for update_text_region (function)
macro_rules! Depcrate_utilsupdate_text_region {
() => {
// Module: crate::utils
// Provides: {"update_text_region"}
// Dependencies: {}
# [doc = " Replaces a region in a text delimited by two strings. Returns the new text if both delimiters"] # [doc = " were found, or the missing delimiter if not."] pub fn update_text_region (path : & Path , start : & str , end : & str , src : & str , dst : & mut String , insert : & mut impl FnMut (& mut String) ,) -> UpdateStatus { let Some ((src_start , src_end)) = src . split_once (start) else { panic ! ("`{}` does not contain `{start}`" , path . display ()) ; } ; let Some ((replaced_text , src_end)) = src_end . split_once (end) else { panic ! ("`{}` does not contain `{end}`" , path . display ()) ; } ; dst . push_str (src_start) ; dst . push_str (start) ; let new_start = dst . len () ; insert (dst) ; let changed = dst [new_start ..] != * replaced_text ; dst . push_str (end) ; dst . push_str (src_end) ; UpdateStatus :: from_changed (changed) }
};
}
