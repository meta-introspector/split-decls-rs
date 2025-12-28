macro_rules! deps {
    () => {
        DFA!();
        Input!();
        Anchored!();
        Automaton!();
        Teddy!();
        PrefilterI!();
        Span!();
    };
}

macro_rules! impl_741 {
    () => {
        deps!();
        impl PrefilterI for Teddy { fn find (& self , haystack : & [u8] , span : Span) -> Option < Span > { # [cfg (not (feature = "perf-literal-multisubstring"))] { unreachable ! () } # [cfg (feature = "perf-literal-multisubstring")] { let ac_span = aho_corasick :: Span { start : span . start , end : span . end } ; self . searcher . find_in (haystack , ac_span) . map (| m | Span { start : m . start () , end : m . end () }) } } fn prefix (& self , haystack : & [u8] , span : Span) -> Option < Span > { # [cfg (not (feature = "perf-literal-multisubstring"))] { unreachable ! () } # [cfg (feature = "perf-literal-multisubstring")] { use aho_corasick :: automaton :: Automaton ; let input = aho_corasick :: Input :: new (haystack) . anchored (aho_corasick :: Anchored :: Yes) . span (span . start .. span . end) ; self . anchored_ac . try_find (& input) . expect ("aho-corasick DFA should never fail") . map (| m | Span { start : m . start () , end : m . end () }) } } fn memory_usage (& self) -> usize { # [cfg (not (feature = "perf-literal-multisubstring"))] { unreachable ! () } # [cfg (feature = "perf-literal-multisubstring")] { use aho_corasick :: automaton :: Automaton ; self . searcher . memory_usage () + self . anchored_ac . memory_usage () } } fn is_fast (& self) -> bool { # [cfg (not (feature = "perf-literal-multisubstring"))] { unreachable ! () } # [cfg (feature = "perf-literal-multisubstring")] { self . minimum_len >= 3 } } }
    };
}

impl_741!()