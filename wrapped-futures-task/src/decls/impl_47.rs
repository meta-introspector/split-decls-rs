macro_rules! deps {
    () => {
        UnsafeFutureObj!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        unsafe impl < 'a , T > UnsafeFutureObj < 'a , T > for & 'a mut (dyn Future < Output = T > + Unpin + 'a) { fn into_raw (self) -> * mut (dyn Future < Output = T > + 'a) { self as * mut dyn Future < Output = T > } unsafe fn drop (_ptr : * mut (dyn Future < Output = T > + 'a)) { } }
    };
}

impl_47!();