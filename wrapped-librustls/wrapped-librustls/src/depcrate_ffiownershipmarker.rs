// Generated macro for OwnershipMarker (trait)
macro_rules! Depcrate_ffiOwnershipMarker {
() => {
// Module: crate::ffi
// Provides: {"OwnershipMarker"}
// Dependencies: {}
# [doc = " A trait for marking the type of a pointer to a [`Castable`]'s underlying `Castable::RustType`"] # [doc = " that is provided to C code, either a [`OwnershipBox`] when it is a pointer to a `Box<_>`,"] # [doc = " a [`OwnershipArc`] when it is a pointer to an `Arc<_>`, or a [`OwnershipRef`] when it is a"] # [doc = " pointer to a `&_`."] # [allow (dead_code)] pub (crate) trait OwnershipMarker { }
};
}
