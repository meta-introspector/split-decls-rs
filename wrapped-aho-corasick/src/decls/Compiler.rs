macro_rules! deps {
    () => {
        Builder!();
        ByteClassSet!();
        NFA!();
    };
}

macro_rules! Compiler {
    () => {
        deps!();
        # [doc = " A compiler uses a builder configuration and builds up the NFA formulation"] # [doc = " of an Aho-Corasick automaton. This roughly corresponds to the standard"] # [doc = " formulation described in textbooks, with some tweaks to support leftmost"] # [doc = " searching."] # [derive (Debug)] struct Compiler < 'a > { builder : & 'a Builder , prefilter : prefilter :: Builder , nfa : NFA , byteset : ByteClassSet , }
    };
}

Compiler!();