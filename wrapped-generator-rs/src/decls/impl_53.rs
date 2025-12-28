macro_rules! deps {
    () => {
        StackBox!();
        StackBoxHeader!();
        Stack!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < T > StackBox < T > { # [doc = " create uninit stack box"] fn new_uninit (stack : & mut Stack , need_drop : usize) -> MaybeUninit < Self > { let _ = stack as * mut Stack ; let offset = unsafe { & mut * stack . get_offset () } ; let layout = std :: alloc :: Layout :: new :: < T > () ; let align = std :: cmp :: max (layout . align () , ALIGN) ; let size = ((layout . size () + align - 1) & ! (align - 1)) / std :: mem :: size_of :: < usize > () ; let u_align = align / std :: mem :: size_of :: < usize > () ; let pad_size = u_align - (* offset + size) % u_align ; let data_size = size + pad_size ; * offset += data_size ; let ptr = unsafe { ptr :: NonNull :: new_unchecked (stack . end () as * mut T) } ; * offset += HEADER_SIZE ; unsafe { let mut header = ptr :: NonNull :: new_unchecked (stack . end () as * mut StackBoxHeader) ; let header = header . as_mut () ; header . data_size = data_size ; header . need_drop = need_drop ; header . stack = stack . shadow_clone () ; MaybeUninit :: new (StackBox { ptr }) } } fn get_header (& self) -> & StackBoxHeader { unsafe { let header = (self . ptr . as_ptr () as * mut usize) . offset (0 - HEADER_SIZE as isize) ; & * (header as * const StackBoxHeader) } } # [doc = " move data into the box"] pub (crate) unsafe fn init (& mut self , data : T) { ptr :: write (self . ptr . as_ptr () , data) ; } pub (crate) fn as_ptr (& self) -> * mut T { self . ptr . as_ptr () } # [doc = " Constructs a StackBox from a raw pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is unsafe because improper use may lead to"] # [doc = " memory problems. For example, a double-free may occur if the"] # [doc = " function is called twice on the same raw pointer."] # [inline] pub (crate) unsafe fn from_raw (raw : * mut T) -> Self { StackBox { ptr : ptr :: NonNull :: new_unchecked (raw) , } } }
    };
}

impl_53!();