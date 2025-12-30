// Generated macro for maybe_resolve_aliases (function)
macro_rules! Depcrate_ir_dfgmaybe_resolve_aliases {
() => {
// Module: crate::ir::dfg
// Provides: {"maybe_resolve_aliases"}
// Dependencies: {}
# [doc = " Resolve value aliases."] # [doc = ""] # [doc = " Find the original SSA value that `value` aliases, or None if an"] # [doc = " alias cycle is detected."] fn maybe_resolve_aliases (values : & PrimaryMap < Value , ValueDataPacked > , value : Value ,) -> Option < Value > { let mut v = value ; for _ in 0 ..= values . len () { if let ValueData :: Alias { original , .. } = ValueData :: from (values [v]) { v = original ; } else { return Some (v) ; } } None }
};
}
