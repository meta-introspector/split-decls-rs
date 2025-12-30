// Generated macro for impl_int (macro)
macro_rules! Depcrate_unstructuredimpl_int {
() => {
// Module: crate::unstructured
// Provides: {"impl_int"}
// Dependencies: {}
macro_rules ! impl_int { ($ ($ ty : ty : $ unsigned_ty : ty ;) *) => { $ (impl Int for $ ty { type Unsigned = $ unsigned_ty ; const ZERO : Self = 0 ; const ONE : Self = 1 ; const MAX : Self = Self :: MAX ; fn from_u8 (b : u8) -> Self { b as Self } fn from_usize (u : usize) -> Self { u as Self } fn checked_add (self , rhs : Self) -> Option < Self > { <$ ty >:: checked_add (self , rhs) } fn wrapping_add (self , rhs : Self) -> Self { <$ ty >:: wrapping_add (self , rhs) } fn wrapping_sub (self , rhs : Self) -> Self { <$ ty >:: wrapping_sub (self , rhs) } fn to_unsigned (self) -> Self :: Unsigned { self as $ unsigned_ty } fn from_unsigned (unsigned : $ unsigned_ty) -> Self { unsigned as Self } }) * } }
};
}
