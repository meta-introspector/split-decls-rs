macro_rules! deps {
    () => {
        Buffer!();
        Integer!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Buffer { # [doc = " This is a cheap operation; you don't need to worry about reusing buffers"] # [doc = " for efficiency."] # [inline] # [cfg_attr (feature = "no-panic" , no_panic)] pub fn new () -> Buffer { let bytes = [MaybeUninit :: < u8 > :: uninit () ; i128 :: MAX_STR_LEN] ; Buffer { bytes } } # [doc = " Print an integer into this buffer and return a reference to its string"] # [doc = " representation within the buffer."] # [cfg_attr (feature = "no-panic" , no_panic)] pub fn format < I : Integer > (& mut self , i : I) -> & str { let string = i . write (unsafe { & mut * (& mut self . bytes as * mut [MaybeUninit < u8 > ; i128 :: MAX_STR_LEN] as * mut < I as private :: Sealed > :: Buffer) }) ; if string . len () > I :: MAX_STR_LEN { unsafe { hint :: unreachable_unchecked () } ; } string } }
    };
}

impl_7!()