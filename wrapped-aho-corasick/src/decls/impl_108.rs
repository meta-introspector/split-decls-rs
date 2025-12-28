macro_rules! deps {
    () => {
        ForceAlgorithm!();
        Builder!();
        Config!();
        RabinKarp!();
        MatchKind!();
        Teddy!();
        Searcher!();
        Fat!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl Config { # [doc = " Create a new default configuration. A default configuration uses"] # [doc = " leftmost-first match semantics."] pub fn new () -> Config { Config { kind : MatchKind :: LeftmostFirst , force : None , only_teddy_fat : None , only_teddy_256bit : None , heuristic_pattern_limits : true , } } # [doc = " Create a packed builder from this configuration. The builder can be"] # [doc = " used to accumulate patterns and create a [`Searcher`] from them."] pub fn builder (& self) -> Builder { Builder :: from_config (self . clone ()) } # [doc = " Set the match semantics for this configuration."] pub fn match_kind (& mut self , kind : MatchKind) -> & mut Config { self . kind = kind ; self } # [doc = " An undocumented method for forcing the use of the Teddy algorithm."] # [doc = ""] # [doc = " This is only exposed for more precise testing and benchmarks. Callers"] # [doc = " should not use it as it is not part of the API stability guarantees of"] # [doc = " this crate."] # [doc (hidden)] pub fn only_teddy (& mut self , yes : bool) -> & mut Config { if yes { self . force = Some (ForceAlgorithm :: Teddy) ; } else { self . force = None ; } self } # [doc = " An undocumented method for forcing the use of the Fat Teddy algorithm."] # [doc = ""] # [doc = " This is only exposed for more precise testing and benchmarks. Callers"] # [doc = " should not use it as it is not part of the API stability guarantees of"] # [doc = " this crate."] # [doc (hidden)] pub fn only_teddy_fat (& mut self , yes : Option < bool >) -> & mut Config { self . only_teddy_fat = yes ; self } # [doc = " An undocumented method for forcing the use of SSE (`Some(false)`) or"] # [doc = " AVX (`Some(true)`) algorithms."] # [doc = ""] # [doc = " This is only exposed for more precise testing and benchmarks. Callers"] # [doc = " should not use it as it is not part of the API stability guarantees of"] # [doc = " this crate."] # [doc (hidden)] pub fn only_teddy_256bit (& mut self , yes : Option < bool >) -> & mut Config { self . only_teddy_256bit = yes ; self } # [doc = " An undocumented method for forcing the use of the Rabin-Karp algorithm."] # [doc = ""] # [doc = " This is only exposed for more precise testing and benchmarks. Callers"] # [doc = " should not use it as it is not part of the API stability guarantees of"] # [doc = " this crate."] # [doc (hidden)] pub fn only_rabin_karp (& mut self , yes : bool) -> & mut Config { if yes { self . force = Some (ForceAlgorithm :: RabinKarp) ; } else { self . force = None ; } self } # [doc = " Request that heuristic limitations on the number of patterns be"] # [doc = " employed. This useful to disable for benchmarking where one wants to"] # [doc = " explore how Teddy performs on large number of patterns even if the"] # [doc = " heuristics would otherwise refuse construction."] # [doc = ""] # [doc = " This is enabled by default."] pub fn heuristic_pattern_limits (& mut self , yes : bool) -> & mut Config { self . heuristic_pattern_limits = yes ; self } }
    };
}

impl_108!()