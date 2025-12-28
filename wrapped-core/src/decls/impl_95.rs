macro_rules! deps {
    () => {
        WeakRefCount!();
        GUID!();
        TearOff!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl WeakRefCount { pub const fn new () -> Self { Self (AtomicIsize :: new (1)) } pub fn add_ref (& self) -> u32 { self . 0 . fetch_update (Ordering :: Relaxed , Ordering :: Relaxed , | count_or_pointer | { bool :: then_some (! is_weak_ref (count_or_pointer) , count_or_pointer + 1) }) . map (| u | u as u32 + 1) . unwrap_or_else (| pointer | unsafe { TearOff :: decode (pointer) . strong_count . add_ref () }) } # [inline (always)] pub fn is_one (& self) -> bool { self . 0 . load (Ordering :: Acquire) == 1 } pub fn release (& self) -> u32 { self . 0 . fetch_update (Ordering :: Release , Ordering :: Relaxed , | count_or_pointer | { bool :: then_some (! is_weak_ref (count_or_pointer) , count_or_pointer - 1) }) . map (| u | u as u32 - 1) . unwrap_or_else (| pointer | unsafe { let tear_off = TearOff :: decode (pointer) ; let remaining = tear_off . strong_count . release () ; if remaining == 0 { TearOff :: WeakRelease (& mut tear_off . weak_vtable as * mut _ as _) ; } remaining }) } # [doc = " # Safety"] pub unsafe fn query (& self , iid : & GUID , object : * mut c_void) -> * mut c_void { unsafe { if iid != & IWeakReferenceSource :: IID { return null_mut () ; } let mut count_or_pointer = self . 0 . load (Ordering :: Relaxed) ; if is_weak_ref (count_or_pointer) { return TearOff :: from_encoding (count_or_pointer) ; } let tear_off = TearOff :: new (object , count_or_pointer as u32) ; let tear_off_ptr : * mut c_void = transmute_copy (& tear_off) ; let encoding : usize = ((tear_off_ptr as usize) >> 1) | (1 << (usize :: BITS - 1)) ; loop { match self . 0 . compare_exchange_weak (count_or_pointer , encoding as isize , Ordering :: AcqRel , Ordering :: Relaxed ,) { Ok (_) => { let result : * mut c_void = transmute (tear_off) ; TearOff :: from_strong_ptr (result) . strong_count . add_ref () ; return result ; } Err (pointer) => count_or_pointer = pointer , } if is_weak_ref (count_or_pointer) { return TearOff :: from_encoding (count_or_pointer) ; } TearOff :: from_strong_ptr (tear_off_ptr) . strong_count . 0 . store (count_or_pointer as i32 , Ordering :: SeqCst) ; } } } }
    };
}

impl_95!();