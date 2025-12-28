macro_rules! WakerRef {
    () => {
        # [doc = " A [`Waker`] that is only valid for a given lifetime."] # [doc = ""] # [doc = " Note: this type implements [`Deref<Target = Waker>`](std::ops::Deref),"] # [doc = " so it can be used to get a `&Waker`."] # [derive (Debug)] pub struct WakerRef < 'a > { waker : ManuallyDrop < Waker > , _marker : PhantomData < & 'a () > , }
    };
}

WakerRef!();