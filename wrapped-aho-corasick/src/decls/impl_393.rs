macro_rules! deps {
    () => {
        PrefilterI!();
        RareBytesTwo!();
        Span!();
        Candidate!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        # [cfg (feature = "perf-literal")] impl PrefilterI for RareBytesTwo { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr2 (self . byte1 , self . byte2 , & haystack [span]) . map (| i | { let pos = span . start + i ; let offset = self . offsets . set [usize :: from (haystack [pos])] . max ; cmp :: max (span . start , pos . saturating_sub (usize :: from (offset))) }) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
    };
}

impl_393!()