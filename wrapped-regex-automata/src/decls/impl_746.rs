macro_rules! deps {
    () => {
        Span!();
        PrefilterI!();
    };
}

macro_rules! impl_746 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < P : PrefilterI + ? Sized > PrefilterI for Arc < P > { # [cfg_attr (feature = "perf-inline" , inline (always))] fn find (& self , haystack : & [u8] , span : Span) -> Option < Span > { (* * self) . find (haystack , span) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn prefix (& self , haystack : & [u8] , span : Span) -> Option < Span > { (* * self) . prefix (haystack , span) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn memory_usage (& self) -> usize { (* * self) . memory_usage () } # [cfg_attr (feature = "perf-inline" , inline (always))] fn is_fast (& self) -> bool { (& * * self) . is_fast () } }
    };
}

impl_746!()