macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T : ? Sized , A : Allocator > From < Box < T , A > > for Pin < Box < T , A > > where A : 'static , { # [doc = " Converts a `Box<T>` into a `Pin<Box<T>>`. If `T` does not implement [`Unpin`], then"] # [doc = " `*boxed` will be pinned in memory and unable to be moved."] # [doc = ""] # [doc = " This conversion does not allocate on the heap and happens in place."] # [doc = ""] # [doc = " This is also available via [`Box::into_pin`]."] # [doc = ""] # [doc = " Constructing and pinning a `Box` with <code><Pin<Box\\<T>>>::from([Box::new]\\(x))</code>"] # [doc = " can also be written more concisely using <code>[Box::pin]\\(x)</code>."] # [doc = " This `From` implementation is useful if you already have a `Box<T>`, or you are"] # [doc = " constructing a (pinned) `Box` in a different way than with [`Box::new`]."] # [inline (always)] fn from (boxed : Box < T , A >) -> Self { Box :: into_pin (boxed) } }
    };
}

impl_39!();