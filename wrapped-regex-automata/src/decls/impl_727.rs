macro_rules! deps {
    () => {
        Memchr!();
        PrefilterI!();
        Span!();
    };
}

macro_rules! impl_727 {
    () => {
        deps!();
        impl PrefilterI for Memchr { fn find (& self , haystack : & [u8] , span : Span) -> Option < Span > { # [cfg (not (feature = "perf-literal-substring"))] { unreachable ! () } # [cfg (feature = "perf-literal-substring")] { memchr :: memchr (self . 0 , & haystack [span]) . map (| i | { let start = span . start + i ; let end = start + 1 ; Span { start , end } }) } } fn prefix (& self , haystack : & [u8] , span : Span) -> Option < Span > { let b = * haystack . get (span . start) ? ; if self . 0 == b { Some (Span { start : span . start , end : span . start + 1 }) } else { None } } fn memory_usage (& self) -> usize { 0 } fn is_fast (& self) -> bool { true } }
    };
}

impl_727!()