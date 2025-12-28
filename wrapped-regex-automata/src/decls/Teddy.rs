macro_rules! deps {
    () => {
        Searcher!();
        DFA!();
    };
}

macro_rules! Teddy {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct Teddy { # [cfg (not (feature = "perf-literal-multisubstring"))] _unused : () , # [doc = " The actual Teddy searcher."] # [doc = ""] # [doc = " Technically, it's possible that Teddy doesn't actually get used, since"] # [doc = " Teddy does require its haystack to at least be of a certain size"] # [doc = " (usually around the size of whatever vector is being used, so ~16"] # [doc = " or ~32 bytes). For haystacks shorter than that, the implementation"] # [doc = " currently uses Rabin-Karp."] # [cfg (feature = "perf-literal-multisubstring")] searcher : aho_corasick :: packed :: Searcher , # [doc = " When running an anchored search, the packed searcher can't handle it so"] # [doc = " we defer to Aho-Corasick itself. Kind of sad, but changing the packed"] # [doc = " searchers to support anchored search would be difficult at worst and"] # [doc = " annoying at best. Since packed searchers only apply to small numbers of"] # [doc = " literals, we content ourselves that this is not much of an added cost."] # [doc = " (That packed searchers only work with a small number of literals is"] # [doc = " also why we use a DFA here. Otherwise, the memory usage of a DFA would"] # [doc = " likely be unacceptable.)"] # [cfg (feature = "perf-literal-multisubstring")] anchored_ac : aho_corasick :: dfa :: DFA , # [doc = " The length of the smallest literal we look for."] # [doc = ""] # [doc = " We use this as a heuristic to figure out whether this will be \"fast\" or"] # [doc = " not. Generally, the longer the better, because longer needles are more"] # [doc = " discriminating and thus reduce false positive rate."] # [cfg (feature = "perf-literal-multisubstring")] minimum_len : usize , }
    };
}

Teddy!();