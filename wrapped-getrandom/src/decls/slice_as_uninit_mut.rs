macro_rules! slice_as_uninit_mut {
    () => {
        # [doc = " View an mutable initialized array as potentially-uninitialized."] # [doc = ""] # [doc = " This is unsafe because it allows assigning uninitialized values into"] # [doc = " `slice`, which would be undefined behavior."] # [inline (always)] # [allow (unused_unsafe)] pub unsafe fn slice_as_uninit_mut < T > (slice : & mut [T]) -> & mut [MaybeUninit < T >] { let ptr = ptr_from_mut :: < [T] > (slice) as * mut [MaybeUninit < T >] ; unsafe { & mut * ptr } }
    };
}

slice_as_uninit_mut!()