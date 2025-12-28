macro_rules! deps {
    () => {
        RareBytesThree!();
        PrefilterI!();
        Candidate!();
        Span!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        # [cfg (feature = "perf-literal")] impl PrefilterI for RareBytesThree { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr3 (self . byte1 , self . byte2 , self . byte3 , & haystack [span]) . map (| i | { let pos = span . start + i ; let offset = self . offsets . set [usize :: from (haystack [pos])] . max ; cmp :: max (span . start , pos . saturating_sub (usize :: from (offset))) }) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
    };
}

impl_395!()