// Generated macro for CANNOT_IMPLICITLY_DEREF_POINTER_TRAIT_OBJ (const)
macro_rules! Depcrate_patCANNOT_IMPLICITLY_DEREF_POINTER_TRAIT_OBJ {
() => {
// Module: crate::pat
// Provides: {"CANNOT_IMPLICITLY_DEREF_POINTER_TRAIT_OBJ"}
// Dependencies: {}
const CANNOT_IMPLICITLY_DEREF_POINTER_TRAIT_OBJ : & str = "\
This error indicates that a pointer to a trait type cannot be implicitly dereferenced by a \
pattern. Every trait defines a type, but because the size of trait implementors isn't fixed, \
this type has no compile-time size. Therefore, all accesses to trait types must be through \
pointers. If you encounter this error you should try to avoid dereferencing the pointer.

You can read more about trait objects in the Trait Objects section of the Reference: \
https://doc.rust-lang.org/reference/types.html#trait-objects" ;
};
}
