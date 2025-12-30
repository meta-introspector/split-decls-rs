// Generated macro for WakerRef (struct)
macro_rules! Depcrate_waker_refWakerRef {
() => {
// Module: crate::waker_ref
// Provides: {"WakerRef"}
// Dependencies: {}
# [doc = " A [`Waker`] that is only valid for a given lifetime."] # [doc = ""] # [doc = " Note: this type implements [`Deref<Target = Waker>`](std::ops::Deref),"] # [doc = " so it can be used to get a `&Waker`."] # [derive (Debug)] pub struct WakerRef < 'a > { waker : ManuallyDrop < Waker > , _marker : PhantomData < & 'a () > , }
};
}
