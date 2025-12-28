macro_rules! deps {
    () => {
        PrefilterI!();
        Span!();
        Candidate!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        impl < P : PrefilterI + ? Sized > PrefilterI for Arc < P > { # [inline (always)] fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { (* * self) . find_in (haystack , span) } }
    };
}

impl_373!()