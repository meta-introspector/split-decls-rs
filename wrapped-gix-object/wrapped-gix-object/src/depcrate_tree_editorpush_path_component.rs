// Generated macro for push_path_component (function)
macro_rules! Depcrate_tree_editorpush_path_component {
() => {
// Module: crate::tree::editor
// Provides: {"push_path_component"}
// Dependencies: {}
fn push_path_component (base : & mut BString , component : & [u8]) -> usize { let prev_len = base . len () ; debug_assert ! (base . last () != Some (& b'/')) ; if ! base . is_empty () { base . push_byte (b'/') ; } base . push_str (component) ; prev_len }
};
}
