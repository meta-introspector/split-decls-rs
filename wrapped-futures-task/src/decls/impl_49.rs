macro_rules! deps {
    () => {
        UnsafeFutureObj!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        unsafe impl < 'a , T > UnsafeFutureObj < 'a , T > for Pin < & 'a mut (dyn Future < Output = T > + 'a) > { fn into_raw (self) -> * mut (dyn Future < Output = T > + 'a) { unsafe { self . get_unchecked_mut () as * mut dyn Future < Output = T > } } unsafe fn drop (_ptr : * mut (dyn Future < Output = T > + 'a)) { } }
    };
}

impl_49!()