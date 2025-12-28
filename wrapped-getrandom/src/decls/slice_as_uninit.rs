macro_rules! slice_as_uninit {
    () => {
        # [inline (always)] pub fn slice_as_uninit < T > (slice : & [T]) -> & [MaybeUninit < T >] { let ptr = ptr_from_ref :: < [T] > (slice) as * const [MaybeUninit < T >] ; unsafe { & * ptr } }
    };
}

slice_as_uninit!();