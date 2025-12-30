// Generated macro for generate_rust_specializations (function)
macro_rules! Depcrate_common_gen_rustgenerate_rust_specializations {
() => {
// Module: crate::common::gen_rust
// Provides: {"generate_rust_specializations"}
// Dependencies: {}
# [doc = " Generate the specializations (unique sequences of const-generic arguments) for this intrinsic."] fn generate_rust_specializations (constraints : & mut impl Iterator < Item = impl Iterator < Item = i64 > > ,) -> Vec < Vec < i32 > > { let mut specializations = vec ! [vec ! []] ; for constraint in constraints { specializations = constraint . flat_map (| right | { specializations . iter () . map (move | left | { let mut left = left . clone () ; left . push (i32 :: try_from (right) . unwrap ()) ; left }) }) . collect () ; } specializations }
};
}
