// Generated macro for ArrayBuilder (struct)
macro_rules! Depcrate_internalArrayBuilder {
() => {
// Module: crate::internal
// Provides: {"ArrayBuilder"}
// Dependencies: {}
# [doc = " **UNSAFE**: Creates an array one element at a time using a mutable iterator of pointers."] # [doc = ""] # [doc = " You MUST increment the position while iterating to mark off created elements,"] # [doc = " which will be dropped if `into_inner` is not called."] # [doc = ""] # [doc = " This is soft-deprecated in favor of [`IntrusiveArrayBuilder`] due to Rust's"] # [doc = " lack of return-value optimization causing issues moving the array in/out of the struct."] # [doc = " Still works fine for smaller arrays, though."] pub struct ArrayBuilder < T , N : ArrayLength > { array : GenericArray < MaybeUninit < T > , N > , position : usize , }
};
}
