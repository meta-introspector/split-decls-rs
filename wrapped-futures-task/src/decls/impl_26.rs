macro_rules! deps {
    () => {
        WakerRef!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a > WakerRef < 'a > { # [doc = " Create a new [`WakerRef`] from a [`Waker`] reference."] # [inline] pub fn new (waker : & 'a Waker) -> Self { let waker = ManuallyDrop :: new (unsafe { core :: ptr :: read (waker) }) ; Self { waker , _marker : PhantomData } } # [doc = " Create a new [`WakerRef`] from a [`Waker`] that must not be dropped."] # [doc = ""] # [doc = " Note: this if for rare cases where the caller created a [`Waker`] in"] # [doc = " an unsafe way (that will be valid only for a lifetime to be determined"] # [doc = " by the caller), and the [`Waker`] doesn't need to or must not be"] # [doc = " destroyed."] # [inline] pub fn new_unowned (waker : ManuallyDrop < Waker >) -> Self { Self { waker , _marker : PhantomData } } }
    };
}

impl_26!();