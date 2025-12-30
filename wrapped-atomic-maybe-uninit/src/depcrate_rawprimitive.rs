// Generated macro for Primitive (trait)
macro_rules! Depcrate_rawPrimitive {
() => {
// Module: crate::raw
// Provides: {"Primitive"}
// Dependencies: {}
# [doc = " Primitive types that may support atomic operations."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside of `atomic-maybe-uninit`."] # [doc = ""] # [doc = " Currently this is implemented only for integer types."] pub trait Primitive : crate :: private :: PrimitivePriv { }
};
}
