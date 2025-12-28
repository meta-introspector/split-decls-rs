macro_rules! deps {
    () => {
        Gen!();
        VecShrinker!();
        Arbitrary!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl Arbitrary for CString { fn arbitrary (g : & mut Gen) -> Self { let size = { let s = g . size () ; g . random_range (0 .. s) } ; let utf8 : bool = g . random () ; if utf8 { CString :: new ((0 ..) . map (| _ | char :: arbitrary (g)) . filter (| & c | c != '\0') . take (size) . collect :: < String > () ,) } else { CString :: new ((0 ..) . map (| _ | u8 :: arbitrary (g)) . filter (| & c | c != b'\0') . take (size) . collect :: < Vec < u8 > > () ,) } . expect ("null characters should have been filtered out") } fn shrink (& self) -> Box < dyn Iterator < Item = CString > > { Box :: new (VecShrinker :: new (self . as_bytes () . to_vec ()) . map (| bytes | { CString :: new (bytes . into_iter () . filter (| & c | c != 0) . collect :: < Vec < u8 > > () ,) . expect ("null characters should have been filtered out") })) } }
    };
}

impl_38!();