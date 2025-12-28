macro_rules! deps {
    () => {
        Box!();
        Unique!();
        Allocator!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < A : Allocator + Default > Default for Box < str , A > { # [inline (always)] fn default () -> Self { let ptr : Unique < str > = unsafe { let bytes : NonNull < [u8] > = NonNull :: < [u8 ; 0] > :: dangling () ; Unique :: new_unchecked (bytes . as_ptr () as * mut str) } ; Box (ptr , A :: default ()) } }
    };
}

impl_29!();