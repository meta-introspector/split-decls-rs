// Generated macro for tests (module)
macro_rules! Depcrate_primitivestests {
() => {
// Module: crate::primitives
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: Word ; # [test] fn carrying_mul_add_cannot_overflow () { let lhs = Word :: MAX ; let rhs = Word :: MAX ; let addend = Word :: MAX ; let carry_in = Word :: MAX ; let (result , carry_out) = super :: carrying_mul_add (lhs , rhs , addend , carry_in) ; assert_eq ! (result , Word :: MAX) ; assert_eq ! (carry_out , Word :: MAX) ; } }
};
}
