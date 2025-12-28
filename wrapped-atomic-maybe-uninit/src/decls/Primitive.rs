macro_rules! Primitive {
    () => {
        # [doc = " Primitive types that may support atomic operations."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside of `atomic-maybe-uninit`."] # [doc = ""] # [doc = " Currently this is implemented only for integer types."] pub trait Primitive : crate :: private :: PrimitivePriv { }
    };
}

Primitive!();