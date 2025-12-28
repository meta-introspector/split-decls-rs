macro_rules! deps {
    () => {
        StartBytesTwo!();
        Span!();
        Candidate!();
        PrefilterI!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        # [cfg (feature = "perf-literal")] impl PrefilterI for StartBytesTwo { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr2 (self . byte1 , self . byte2 , & haystack [span]) . map (| i | span . start + i) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
    };
}

impl_401!()