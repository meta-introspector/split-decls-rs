// Generated macro for Ref (struct)
macro_rules! Depcrate_ptrRef {
() => {
// Module: crate::ptr
// Provides: {"Ref"}
// Dependencies: {}
# [repr (transparent)] pub struct Ref < 'a , T > where T : ? Sized , { pub ptr : NonNull < T > , lifetime : PhantomData < & 'a T > , }
};
}
