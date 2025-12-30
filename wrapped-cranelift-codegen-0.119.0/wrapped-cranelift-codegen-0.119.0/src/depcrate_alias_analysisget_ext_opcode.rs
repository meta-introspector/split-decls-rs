// Generated macro for get_ext_opcode (function)
macro_rules! Depcrate_alias_analysisget_ext_opcode {
() => {
// Module: crate::alias_analysis
// Provides: {"get_ext_opcode"}
// Dependencies: {}
fn get_ext_opcode (op : Opcode) -> Option < Opcode > { debug_assert ! (op . can_load () || op . can_store ()) ; match op { Opcode :: Load | Opcode :: Store => None , _ => Some (op) , } }
};
}
