macro_rules! AhoCorasick {
    () => {
        # [derive (Clone , Debug)] pub (crate) struct AhoCorasick { # [cfg (not (feature = "perf-literal-multisubstring"))] _unused : () , # [cfg (feature = "perf-literal-multisubstring")] ac : aho_corasick :: AhoCorasick , }
    };
}

AhoCorasick!();