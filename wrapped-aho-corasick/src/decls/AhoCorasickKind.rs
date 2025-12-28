macro_rules! deps {
    () => {
        NFA!();
        AhoCorasick!();
        DFA!();
    };
}

macro_rules! AhoCorasickKind {
    () => {
        deps!();
        # [doc = " The type of Aho-Corasick implementation to use in an [`AhoCorasick`]"] # [doc = " searcher."] # [doc = ""] # [doc = " This is principally used as an input to the"] # [doc = " [`AhoCorasickBuilder::start_kind`] method. Its documentation goes into more"] # [doc = " detail about each choice."] # [non_exhaustive] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum AhoCorasickKind { # [doc = " Use a noncontiguous NFA."] NoncontiguousNFA , # [doc = " Use a contiguous NFA."] ContiguousNFA , # [doc = " Use a DFA. Warning: DFAs typically use a large amount of memory."] DFA , }
    };
}

AhoCorasickKind!()