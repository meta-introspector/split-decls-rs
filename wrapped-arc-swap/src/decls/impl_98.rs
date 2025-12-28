macro_rules! deps {
    () => {
        RefCnt!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        unsafe impl < T : RefCnt > RefCnt for Option < T > { type Base = T :: Base ; fn into_ptr (me : Option < T >) -> * mut T :: Base { me . map (T :: into_ptr) . unwrap_or_else (ptr :: null_mut) } fn as_ptr (me : & Option < T >) -> * mut T :: Base { me . as_ref () . map (T :: as_ptr) . unwrap_or_else (ptr :: null_mut) } unsafe fn from_ptr (ptr : * const T :: Base) -> Option < T > { if ptr . is_null () { None } else { Some (T :: from_ptr (ptr)) } } }
    };
}

impl_98!();