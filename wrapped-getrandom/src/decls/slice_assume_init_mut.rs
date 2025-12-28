macro_rules! slice_assume_init_mut {
    () => {
        # [doc = " Polyfill for `maybe_uninit_slice` feature's"] # [doc = " `MaybeUninit::slice_assume_init_mut`. Every element of `slice` must have"] # [doc = " been initialized."] # [inline (always)] # [allow (unused_unsafe)] pub unsafe fn slice_assume_init_mut < T > (slice : & mut [MaybeUninit < T >]) -> & mut [T] { let ptr = ptr_from_mut :: < [MaybeUninit < T >] > (slice) as * mut [T] ; unsafe { & mut * ptr } }
    };
}

slice_assume_init_mut!()