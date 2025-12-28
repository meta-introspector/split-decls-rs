macro_rules! Stride {
    () => {
        # [doc = " (the stride) skipped per iteration."] # [doc = ""] # [doc = " `Stride` does not support zero-sized types for `A`."] # [doc = ""] # [doc = " Iterator element type is `&'a A`."] pub struct Stride < 'a , A : 'a > { # [doc = " base pointer -- does not change during iteration"] begin : * const A , # [doc = " current offset from begin"] offset : isize , # [doc = " offset where we end (exclusive end)."] end : isize , stride : isize , life : marker :: PhantomData < & 'a A > , }
    };
}

Stride!()