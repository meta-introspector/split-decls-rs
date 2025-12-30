// Generated macro for Mut (struct)
macro_rules! Depcrate_ptrMut {
() => {
// Module: crate::ptr
// Provides: {"Mut"}
// Dependencies: {}
# [repr (transparent)] pub struct Mut < 'a , T > where T : ? Sized , { pub ptr : NonNull < T > , lifetime : PhantomData < & 'a mut T > , }
};
}
