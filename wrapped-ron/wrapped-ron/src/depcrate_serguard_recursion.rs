// Generated macro for guard_recursion (macro)
macro_rules! Depcrate_serguard_recursion {
() => {
// Module: crate::ser
// Provides: {"guard_recursion"}
// Dependencies: {}
macro_rules ! guard_recursion { ($ self : expr => $ expr : expr) => { { if let Some (limit) = & mut $ self . recursion_limit { if let Some (new_limit) = limit . checked_sub (1) { * limit = new_limit ; } else { return Err (Error :: ExceededRecursionLimit) ; } } let result = $ expr ; if let Some (limit) = & mut $ self . recursion_limit { * limit = limit . saturating_add (1) ; } result } } ; }
};
}
