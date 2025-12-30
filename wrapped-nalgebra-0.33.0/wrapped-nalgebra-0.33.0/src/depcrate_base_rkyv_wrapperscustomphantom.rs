// Generated macro for CustomPhantom (struct)
macro_rules! Depcrate_base_rkyv_wrappersCustomPhantom {
() => {
// Module: crate::base::rkyv_wrappers
// Provides: {"CustomPhantom"}
// Dependencies: {}
# [doc = " A wrapper that allows for changing the generic type of a `PhantomData<T>`."] pub struct CustomPhantom < NT : ? Sized > { _data : PhantomData < * const NT > , }
};
}
