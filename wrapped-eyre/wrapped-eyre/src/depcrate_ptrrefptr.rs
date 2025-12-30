// Generated macro for RefPtr (struct)
macro_rules! Depcrate_ptrRefPtr {
() => {
// Module: crate::ptr
// Provides: {"RefPtr"}
// Dependencies: {}
# [doc = " Convenience lifetime annotated mutable pointer which facilitates returning an inferred lifetime"] # [doc = " in a `fn` pointer."] pub (crate) struct RefPtr < 'a , T : ? Sized > { pub (crate) ptr : NonNull < T > , _marker : PhantomData < & 'a T > , }
};
}
