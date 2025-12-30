// Generated macro for impl_23 (impl)
macro_rules! Depcrate_lineimpl_23 {
() => {
// Module: crate::line
// Provides: {"impl_23"}
// Dependencies: {}
impl TimeType { fn from_char (c : char) -> Option < Self > { Some (match c { 'w' => Self :: Wall , 's' => Self :: Standard , 'u' | 'g' | 'z' => Self :: UTC , _ => return None , }) } }
};
}
