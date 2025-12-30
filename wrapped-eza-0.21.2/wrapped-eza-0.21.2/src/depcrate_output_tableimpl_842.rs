// Generated macro for impl_842 (impl)
macro_rules! Depcrate_output_tableimpl_842 {
() => {
// Module: crate::output::table
// Provides: {"impl_842"}
// Dependencies: {}
impl FlagsFormat { pub (crate) fn deduce < V : Vars > (vars : & V) -> FlagsFormat { vars . get (EZA_WINDOWS_ATTRIBUTES) . and_then (| v | match v . to_ascii_lowercase () . to_str () { Some ("short") => Some (FlagsFormat :: Short) , Some ("long") => Some (FlagsFormat :: Long) , _ => None , }) . unwrap_or_default () } }
};
}
