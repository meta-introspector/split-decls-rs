macro_rules! deps {
    () => {
        Candidate!();
        Span!();
        StartBytesOne!();
        PrefilterI!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        # [cfg (feature = "perf-literal")] impl PrefilterI for StartBytesOne { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr (self . byte1 , & haystack [span]) . map (| i | span . start + i) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
    };
}

impl_399!();