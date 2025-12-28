macro_rules! deps {
    () => {
        Span!();
        PrefilterI!();
        Candidate!();
        PatternID!();
        Memmem!();
        Match!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        # [cfg (all (feature = "std" , feature = "perf-literal"))] impl PrefilterI for Memmem { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { use crate :: util :: primitives :: PatternID ; self . 0 . find (& haystack [span]) . map_or (Candidate :: None , | i | { let start = span . start + i ; let end = start + self . 0 . needle () . len () ; Candidate :: Match (Match :: new (PatternID :: ZERO , start .. end)) }) } }
    };
}

impl_381!()