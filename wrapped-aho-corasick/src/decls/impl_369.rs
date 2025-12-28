macro_rules! deps {
    () => {
        Prefilter!();
        Candidate!();
        Span!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl Prefilter { # [doc = " Execute a search in the haystack within the span given. If a match or"] # [doc = " a possible match is returned, then it is guaranteed to occur within"] # [doc = " the bounds of the span."] # [doc = ""] # [doc = " If the span provided is invalid for the given haystack, then behavior"] # [doc = " is unspecified."] # [inline] pub fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { self . finder . find_in (haystack , span) } # [inline] pub (crate) fn memory_usage (& self) -> usize { self . memory_usage } }
    };
}

impl_369!();