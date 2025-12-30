// Generated macro for PointerCast (enum)
macro_rules! Depcrate_inferPointerCast {
() => {
// Module: crate::infer
// Provides: {"PointerCast"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum PointerCast { # [doc = " Go from a fn-item type to a fn-pointer type."] ReifyFnPointer , # [doc = " Go from a safe fn pointer to an unsafe fn pointer."] UnsafeFnPointer , # [doc = " Go from a non-capturing closure to an fn pointer or an unsafe fn pointer."] # [doc = " It cannot convert a closure that requires unsafe."] ClosureFnPointer (Safety) , # [doc = " Go from a mut raw pointer to a const raw pointer."] MutToConstPointer , # [allow (dead_code)] # [doc = " Go from `*const [T; N]` to `*const T`"] ArrayToPointer , # [doc = " Unsize a pointer/reference value, e.g., `&[T; n]` to"] # [doc = " `&[T]`. Note that the source could be a thin or fat pointer."] # [doc = " This will do things like convert thin pointers to fat"] # [doc = " pointers, or convert structs containing thin pointers to"] # [doc = " structs containing fat pointers, or convert between fat"] # [doc = " pointers. We don't store the details of how the transform is"] # [doc = " done (in fact, we don't know that, because it might depend on"] # [doc = " the precise type parameters). We just store the target"] # [doc = " type. Codegen backends and miri figure out what has to be done"] # [doc = " based on the precise source/target type at hand."] Unsize , }
};
}
