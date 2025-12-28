macro_rules! deps {
    () => {
        BStr!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl BStr { # [doc = " Directly creates a `BStr` slice from anything that can be converted"] # [doc = " to a byte slice."] # [doc = ""] # [doc = " This is very similar to the [`B`](crate::B) function, except this"] # [doc = " returns a `&BStr` instead of a `&[u8]`."] # [doc = ""] # [doc = " This is a cost-free conversion."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " You can create `BStr`'s from byte arrays, byte slices or even string"] # [doc = " slices:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::BStr;"] # [doc = ""] # [doc = " let a = BStr::new(b\"abc\");"] # [doc = " let b = BStr::new(&b\"abc\"[..]);"] # [doc = " let c = BStr::new(\"abc\");"] # [doc = ""] # [doc = " assert_eq!(a, b);"] # [doc = " assert_eq!(a, c);"] # [doc = " ```"] # [inline] pub fn new < B : ? Sized + AsRef < [u8] > > (bytes : & B) -> & BStr { BStr :: from_bytes (bytes . as_ref ()) } # [inline] pub (crate) fn new_mut < B : ? Sized + AsMut < [u8] > > (bytes : & mut B ,) -> & mut BStr { BStr :: from_bytes_mut (bytes . as_mut ()) } # [inline] pub (crate) fn from_bytes (slice : & [u8]) -> & BStr { unsafe { & * (slice as * const [u8] as * const BStr) } } # [inline] pub (crate) fn from_bytes_mut (slice : & mut [u8]) -> & mut BStr { unsafe { & mut * (slice as * mut [u8] as * mut BStr) } } # [inline] # [cfg (feature = "alloc")] pub (crate) fn from_boxed_bytes (slice : Box < [u8] >) -> Box < BStr > { unsafe { Box :: from_raw (Box :: into_raw (slice) as _) } } # [inline] # [cfg (feature = "alloc")] pub (crate) fn into_boxed_bytes (slice : Box < BStr >) -> Box < [u8] > { unsafe { Box :: from_raw (Box :: into_raw (slice) as _) } } # [inline] pub (crate) fn as_bytes (& self) -> & [u8] { & self . bytes } }
    };
}

impl_19!();