macro_rules! deps {
    () => {
        PrefilterI!();
        Span!();
        StartBytesThree!();
        Candidate!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        # [cfg (feature = "perf-literal")] impl PrefilterI for StartBytesThree { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr3 (self . byte1 , self . byte2 , self . byte3 , & haystack [span]) . map (| i | span . start + i) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
    };
}

impl_403!()