// Generated macro for parent_cv (function)
macro_rules! Depcrate_gutsparent_cv {
() => {
// Module: crate::guts
// Provides: {"parent_cv"}
// Dependencies: {}
pub fn parent_cv (left_child : & crate :: Hash , right_child : & crate :: Hash , is_root : bool ,) -> crate :: Hash { let output = crate :: parent_node_output (left_child . as_bytes () , right_child . as_bytes () , crate :: IV , 0 , crate :: platform :: Platform :: detect () ,) ; if is_root { output . root_hash () } else { output . chaining_value () . into () } }
};
}
