// Generated macro for human_readable_changes (function)
macro_rules! Depcrate_utils_change_trackerhuman_readable_changes {
() => {
// Module: crate::utils::change_tracker
// Provides: {"human_readable_changes"}
// Dependencies: {}
pub fn human_readable_changes (changes : & [ChangeInfo]) -> String { let mut message = String :: new () ; for change in changes { message . push_str (& format ! ("  [{}] {}\n" , change . severity , change . summary)) ; message . push_str (& format ! ("    - PR Link https://github.com/rust-lang/rust/pull/{}\n" , change . change_id)) ; } message }
};
}
