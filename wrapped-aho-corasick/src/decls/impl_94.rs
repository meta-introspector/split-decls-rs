macro_rules! deps {
    () => {
        Compiler!();
        NFA!();
        Builder!();
        BuildError!();
        MatchKind!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl Builder { # [doc = " Create a new builder for configuring an Aho-Corasick noncontiguous NFA."] pub fn new () -> Builder { Builder :: default () } # [doc = " Build an Aho-Corasick noncontiguous NFA from the given iterator of"] # [doc = " patterns."] # [doc = ""] # [doc = " A builder may be reused to create more NFAs."] pub fn build < I , P > (& self , patterns : I) -> Result < NFA , BuildError > where I : IntoIterator < Item = P > , P : AsRef < [u8] > , { debug ! ("building non-contiguous NFA") ; let nfa = Compiler :: new (self) ? . compile (patterns) ? ; debug ! ("non-contiguous NFA built, <states: {:?}, size: {:?}>" , nfa . states . len () , nfa . memory_usage ()) ; Ok (nfa) } # [doc = " Set the desired match semantics."] # [doc = ""] # [doc = " See"] # [doc = " [`AhoCorasickBuilder::match_kind`](crate::AhoCorasickBuilder::match_kind)"] # [doc = " for more documentation and examples."] pub fn match_kind (& mut self , kind : MatchKind) -> & mut Builder { self . match_kind = kind ; self } # [doc = " Enable ASCII-aware case insensitive matching."] # [doc = ""] # [doc = " See"] # [doc = " [`AhoCorasickBuilder::ascii_case_insensitive`](crate::AhoCorasickBuilder::ascii_case_insensitive)"] # [doc = " for more documentation and examples."] pub fn ascii_case_insensitive (& mut self , yes : bool) -> & mut Builder { self . ascii_case_insensitive = yes ; self } # [doc = " Set the limit on how many states use a dense representation for their"] # [doc = " transitions. Other states will generally use a sparse representation."] # [doc = ""] # [doc = " See"] # [doc = " [`AhoCorasickBuilder::dense_depth`](crate::AhoCorasickBuilder::dense_depth)"] # [doc = " for more documentation and examples."] pub fn dense_depth (& mut self , depth : usize) -> & mut Builder { self . dense_depth = depth ; self } # [doc = " Enable heuristic prefilter optimizations."] # [doc = ""] # [doc = " See"] # [doc = " [`AhoCorasickBuilder::prefilter`](crate::AhoCorasickBuilder::prefilter)"] # [doc = " for more documentation and examples."] pub fn prefilter (& mut self , yes : bool) -> & mut Builder { self . prefilter = yes ; self } }
    };
}

impl_94!();