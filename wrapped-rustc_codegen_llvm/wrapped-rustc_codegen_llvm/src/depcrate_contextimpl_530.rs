// Generated macro for impl_530 (impl)
macro_rules! Depcrate_contextimpl_530 {
() => {
// Module: crate::context
// Provides: {"impl_530"}
// Dependencies: {}
impl CodegenCx < '_ , '_ > { # [doc = " Generates a new symbol name with the given prefix. This symbol name must"] # [doc = " only be used for definitions with `internal` or `private` linkage."] pub (crate) fn generate_local_symbol_name (& self , prefix : & str) -> String { let idx = self . local_gen_sym_counter . get () ; self . local_gen_sym_counter . set (idx + 1) ; let mut name = String :: with_capacity (prefix . len () + 6) ; name . push_str (prefix) ; name . push ('.') ; name . push_str (& (idx as u64) . to_base (ALPHANUMERIC_ONLY)) ; name } }
};
}
