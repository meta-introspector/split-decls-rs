// Generated macro for Bound (struct)
macro_rules! Depcrate_needless_maybe_sizedBound {
() => {
// Module: crate::needless_maybe_sized
// Provides: {"Bound"}
// Dependencies: {}
# [expect (clippy :: struct_field_names)] struct Bound < 'tcx > { # [doc = " The [`DefId`] of the type parameter the bound refers to"] param : DefId , ident : Ident , trait_bound : & 'tcx PolyTraitRef < 'tcx > , predicate_pos : usize , bound_pos : usize , }
};
}
