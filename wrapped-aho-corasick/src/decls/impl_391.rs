macro_rules! deps {
    () => {
        Candidate!();
        Span!();
        RareBytesOne!();
        PrefilterI!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        # [cfg (feature = "perf-literal")] impl PrefilterI for RareBytesOne { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr (self . byte1 , & haystack [span]) . map (| i | { let pos = span . start + i ; cmp :: max (span . start , pos . saturating_sub (usize :: from (self . offset . max)) ,) }) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
    };
}

impl_391!();