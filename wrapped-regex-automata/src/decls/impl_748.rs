macro_rules! deps {
    () => {
        Memchr2!();
        Memchr3!();
        ByteSet!();
        Memmem!();
        AhoCorasick!();
        Memchr!();
        Teddy!();
        MatchKind!();
        Choice!();
    };
}

macro_rules! impl_748 {
    () => {
        deps!();
        impl Choice { # [doc = " Select what is believed to be the best prefilter algorithm for the"] # [doc = " match semantics and sequence of needles given."] # [doc = ""] # [doc = " This selection algorithm uses the needles as given without any"] # [doc = " modification. For example, if `[bar]` is given, then this doesn't"] # [doc = " try to select `memchr` for `b`. Instead, it would select `memmem`"] # [doc = " for `bar`. If callers would want `memchr` selected for `[bar]`, then"] # [doc = " callers should massages the literals themselves. That is, callers are"] # [doc = " responsible for heuristics surrounding which sequence of literals is"] # [doc = " best."] # [doc = ""] # [doc = " What this selection algorithm does is attempt to use the fastest"] # [doc = " prefilter that works for the literals given. So if `[a, b]`, is given,"] # [doc = " then `memchr2` is selected."] # [doc = ""] # [doc = " Of course, which prefilter is selected is also subject to what"] # [doc = " is available. For example, if `alloc` isn't enabled, then"] # [doc = " that limits which prefilters can be selected. Similarly, if"] # [doc = " `perf-literal-substring` isn't enabled, then nothing from the `memchr`"] # [doc = " crate can be returned."] pub (crate) fn new < B : AsRef < [u8] > > (kind : MatchKind , needles : & [B] ,) -> Option < Choice > { if needles . len () == 0 { debug ! ("prefilter building failed: found empty set of literals") ; return None ; } if needles . iter () . any (| n | n . as_ref () . is_empty ()) { debug ! ("prefilter building failed: literals match empty string") ; return None ; } if let Some (pre) = Memchr :: new (kind , needles) { debug ! ("prefilter built: memchr") ; return Some (Choice :: Memchr (pre)) ; } if let Some (pre) = Memchr2 :: new (kind , needles) { debug ! ("prefilter built: memchr2") ; return Some (Choice :: Memchr2 (pre)) ; } if let Some (pre) = Memchr3 :: new (kind , needles) { debug ! ("prefilter built: memchr3") ; return Some (Choice :: Memchr3 (pre)) ; } if let Some (pre) = Memmem :: new (kind , needles) { debug ! ("prefilter built: memmem") ; return Some (Choice :: Memmem (pre)) ; } if let Some (pre) = Teddy :: new (kind , needles) { debug ! ("prefilter built: teddy") ; return Some (Choice :: Teddy (pre)) ; } if let Some (pre) = ByteSet :: new (kind , needles) { debug ! ("prefilter built: byteset") ; return Some (Choice :: ByteSet (pre)) ; } if let Some (pre) = AhoCorasick :: new (kind , needles) { debug ! ("prefilter built: aho-corasick") ; return Some (Choice :: AhoCorasick (pre)) ; } debug ! ("prefilter building failed: no strategy could be found") ; None } }
    };
}

impl_748!()