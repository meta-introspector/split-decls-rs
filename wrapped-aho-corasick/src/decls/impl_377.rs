macro_rules! deps {
    () => {
        Candidate!();
        Match!();
        PrefilterI!();
        Span!();
        Packed!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl PrefilterI for Packed { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { self . 0 . find_in (haystack , span) . map_or (Candidate :: None , Candidate :: Match) } }
    };
}

impl_377!();