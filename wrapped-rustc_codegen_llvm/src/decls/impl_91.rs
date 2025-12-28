macro_rules! deps {
    () => {
        ThinBuffer!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl ThinBuffer { pub (crate) fn new (m : & llvm :: Module , is_thin : bool) -> ThinBuffer { unsafe { let buffer = llvm :: LLVMRustThinLTOBufferCreate (m , is_thin) ; ThinBuffer (buffer) } } pub (crate) unsafe fn from_raw_ptr (ptr : * mut llvm :: ThinLTOBuffer) -> ThinBuffer { let mut ptr = NonNull :: new (ptr) . unwrap () ; ThinBuffer (unsafe { ptr . as_mut () }) } pub (crate) fn thin_link_data (& self) -> & [u8] { unsafe { let ptr = llvm :: LLVMRustThinLTOBufferThinLinkDataPtr (self . 0) as * const _ ; let len = llvm :: LLVMRustThinLTOBufferThinLinkDataLen (self . 0) ; slice :: from_raw_parts (ptr , len) } } }
    };
}

impl_91!();