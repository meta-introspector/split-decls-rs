macro_rules! slice_assume_init_mut {
    () => {
        # [doc = " Assuming all the elements are initialized, get a mutable slice to them."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It is up to the caller to guarantee that the `MaybeUninit<T>` elements"] # [doc = " really are in an initialized state."] # [doc = " Calling this when the content is not yet fully initialized causes undefined behavior."] # [doc = ""] # [doc = " See [`assume_init_mut`] for more details and examples."] # [doc = ""] # [doc = " [`assume_init_mut`]: MaybeUninit::assume_init_mut"] # [inline (always)] pub unsafe fn slice_assume_init_mut < T > (slice : & mut [MaybeUninit < T >]) -> & mut [T] { unsafe { & mut * (slice as * mut [MaybeUninit < T >] as * mut [T]) } }
    };
}

slice_assume_init_mut!();