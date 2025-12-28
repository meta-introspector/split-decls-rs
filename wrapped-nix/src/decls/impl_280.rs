macro_rules! deps {
    () => {
        Result!();
        NixPath!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl NixPath for [u8] { fn is_empty (& self) -> bool { self . is_empty () } fn len (& self) -> usize { self . len () } fn with_nix_path < T , F > (& self , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { const MAX_STACK_ALLOCATION : usize = 1024 ; if self . len () >= MAX_STACK_ALLOCATION { return with_nix_path_allocating (self , f) ; } let mut buf = MaybeUninit :: < [u8 ; MAX_STACK_ALLOCATION] > :: uninit () ; let buf_ptr = buf . as_mut_ptr () . cast () ; unsafe { ptr :: copy_nonoverlapping (self . as_ptr () , buf_ptr , self . len ()) ; buf_ptr . add (self . len ()) . write (0) ; } match CStr :: from_bytes_with_nul (unsafe { slice :: from_raw_parts (buf_ptr , self . len () + 1) }) { Ok (s) => Ok (f (s)) , Err (_) => Err (Errno :: EINVAL) , } } }
    };
}

impl_280!();