// Generated macro for resolve_aliases (function)
macro_rules! Depcrate_ir_dfgresolve_aliases {
() => {
// Module: crate::ir::dfg
// Provides: {"resolve_aliases"}
// Dependencies: {}
# [doc = " Resolve value aliases."] # [doc = ""] # [doc = " Find the original SSA value that `value` aliases."] fn resolve_aliases (values : & PrimaryMap < Value , ValueDataPacked > , value : Value) -> Value { if let Some (v) = maybe_resolve_aliases (values , value) { v } else { panic ! ("Value alias loop detected for {value}") ; } }
};
}
