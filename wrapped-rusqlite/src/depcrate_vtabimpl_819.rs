// Generated macro for impl_819 (impl)
macro_rules! Depcrate_vtabimpl_819 {
() => {
// Module: crate::vtab
// Provides: {"impl_819"}
// Dependencies: {}
impl IndexConstraint < '_ > { # [doc = " Column constrained.  -1 for ROWID"] # [inline] # [must_use] pub fn column (& self) -> c_int { self . 0 . iColumn } # [doc = " Constraint operator"] # [inline] # [must_use] pub fn operator (& self) -> IndexConstraintOp { IndexConstraintOp :: from (self . 0 . op) } # [doc = " True if this constraint is usable"] # [inline] # [must_use] pub fn is_usable (& self) -> bool { self . 0 . usable != 0 } }
};
}
