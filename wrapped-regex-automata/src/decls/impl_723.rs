macro_rules! deps {
    () => {
        ByteSet!();
        PrefilterI!();
        Span!();
    };
}

macro_rules! impl_723 {
    () => {
        deps!();
        impl PrefilterI for ByteSet { fn find (& self , haystack : & [u8] , span : Span) -> Option < Span > { haystack [span] . iter () . position (| & b | self . 0 [usize :: from (b)]) . map (| i | { let start = span . start + i ; let end = start + 1 ; Span { start , end } }) } fn prefix (& self , haystack : & [u8] , span : Span) -> Option < Span > { let b = * haystack . get (span . start) ? ; if self . 0 [usize :: from (b)] { Some (Span { start : span . start , end : span . start + 1 }) } else { None } } fn memory_usage (& self) -> usize { 0 } fn is_fast (& self) -> bool { false } }
    };
}

impl_723!();