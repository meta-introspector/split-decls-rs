macro_rules! deps {
    () => {
        RefCnt!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        unsafe impl < T > RefCnt for RcWeak < T > { type Base = T ; fn as_ptr (me : & Self) -> * mut T { if RcWeak :: ptr_eq (& RcWeak :: new () , me) { ptr :: null_mut () } else { RcWeak :: as_ptr (me) as * mut T } } fn into_ptr (me : Self) -> * mut T { if RcWeak :: ptr_eq (& RcWeak :: new () , & me) { ptr :: null_mut () } else { RcWeak :: into_raw (me) as * mut T } } unsafe fn from_ptr (ptr : * const T) -> Self { if ptr . is_null () { RcWeak :: new () } else { RcWeak :: from_raw (ptr) } } }
    };
}

impl_134!();