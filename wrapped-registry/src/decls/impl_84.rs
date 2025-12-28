macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Data { pub fn new (len : usize) -> Self { unsafe { let bytes = Self :: alloc (len) ; if len > 0 { core :: ptr :: write_bytes (bytes . ptr , 0 , len) ; } bytes } } pub fn as_wide (& self) -> & [u16] { if self . ptr . is_null () { & [] } else { unsafe { core :: slice :: from_raw_parts (self . ptr as * const u16 , self . len / 2) } } } pub fn from_slice (slice : & [u8]) -> Self { unsafe { let bytes = Self :: alloc (slice . len ()) ; if ! slice . is_empty () { core :: ptr :: copy_nonoverlapping (slice . as_ptr () , bytes . ptr , slice . len ()) ; } bytes } } unsafe fn alloc (len : usize) -> Self { if len == 0 { Self { ptr : null_mut () , len : 0 , } } else { let ptr = unsafe { HeapAlloc (GetProcessHeap () , 0 , len) as * mut u8 } ; if ptr . is_null () { panic ! ("allocation failed") ; } Self { ptr , len } } } }
    };
}

impl_84!();