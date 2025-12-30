// Generated macro for CastInto (trait)
macro_rules! Depcrate_math_support_int_traitsCastInto {
() => {
// Module: crate::math::support::int_traits
// Provides: {"CastInto"}
// Dependencies: {}
# [doc = " Trait to express (possibly lossy) casting of integers"] pub trait CastInto < T : Copy > : Copy { # [doc = " By default, casts should be exact."] # [track_caller] fn cast (self) -> T ; # [doc = " Call for casts that are expected to truncate."] # [doc = ""] # [doc = " In practice, this is exactly the same as `cast`; the main difference is to document intent"] # [doc = " in code. `cast` may panic in debug mode."] fn cast_lossy (self) -> T ; }
};
}
