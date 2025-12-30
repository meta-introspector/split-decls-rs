// Generated macro for MutPtr (struct)
macro_rules! Depcrate_ptrMutPtr {
() => {
// Module: crate::ptr
// Provides: {"MutPtr"}
// Dependencies: {}
# [doc = " Convenience lifetime annotated mutable pointer which facilitates returning an inferred lifetime"] # [doc = " in a `fn` pointer."] pub (crate) struct MutPtr < 'a , T : ? Sized > { pub (crate) ptr : NonNull < T > , _marker : PhantomData < & 'a mut T > , }
};
}
