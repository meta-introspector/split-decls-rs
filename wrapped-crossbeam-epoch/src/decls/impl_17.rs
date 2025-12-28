macro_rules! deps {
    () => {
        Pointable!();
        Array!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T > Pointable for [MaybeUninit < T >] { const ALIGN : usize = mem :: align_of :: < Array < T > > () ; type Init = usize ; unsafe fn init (len : Self :: Init) -> * mut () { let layout = Array :: < T > :: layout (len) ; unsafe { let ptr = alloc :: alloc :: alloc (layout) . cast :: < Array < T > > () ; if ptr . is_null () { alloc :: alloc :: handle_alloc_error (layout) ; } ptr :: addr_of_mut ! ((* ptr) . len) . write (len) ; ptr . cast :: < () > () } } unsafe fn deref < 'a > (ptr : * mut ()) -> & 'a Self { unsafe { let array = & * (ptr as * const Array < T >) ; slice :: from_raw_parts (array . elements . as_ptr () , array . len) } } unsafe fn deref_mut < 'a > (ptr : * mut ()) -> & 'a mut Self { unsafe { let array = & mut * ptr . cast :: < Array < T > > () ; slice :: from_raw_parts_mut (array . elements . as_mut_ptr () , array . len) } } unsafe fn drop (ptr : * mut ()) { unsafe { let len = (* ptr . cast :: < Array < T > > ()) . len ; let layout = Array :: < T > :: layout (len) ; alloc :: alloc :: dealloc (ptr . cast :: < u8 > () , layout) ; } } }
    };
}

impl_17!()