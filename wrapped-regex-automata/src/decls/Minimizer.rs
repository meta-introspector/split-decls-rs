macro_rules! deps {
    () => {
        NFA!();
        DFA!();
        StateID!();
        OwnedDFA!();
        StateSet!();
    };
}

macro_rules! Minimizer {
    () => {
        deps!();
        # [doc = " An implementation of Hopcroft's algorithm for minimizing DFAs."] # [doc = ""] # [doc = " The algorithm implemented here is mostly taken from Wikipedia:"] # [doc = " https://en.wikipedia.org/wiki/DFA_minimization#Hopcroft's_algorithm"] # [doc = ""] # [doc = " This code has had some light optimization attention paid to it,"] # [doc = " particularly in the form of reducing allocation as much as possible."] # [doc = " However, it is still generally slow. Future optimization work should"] # [doc = " probably focus on the bigger picture rather than micro-optimizations. For"] # [doc = " example:"] # [doc = ""] # [doc = " 1. Figure out how to more intelligently create initial partitions. That is,"] # [doc = "    Hopcroft's algorithm starts by creating two partitions of DFA states"] # [doc = "    that are known to NOT be equivalent: match states and non-match states."] # [doc = "    The algorithm proceeds by progressively refining these partitions into"] # [doc = "    smaller partitions. If we could start with more partitions, then we"] # [doc = "    could reduce the amount of work that Hopcroft's algorithm needs to do."] # [doc = " 2. For every partition that we visit, we find all incoming transitions to"] # [doc = "    every state in the partition for *every* element in the alphabet. (This"] # [doc = "    is why using byte classes can significantly decrease minimization times,"] # [doc = "    since byte classes shrink the alphabet.) This is quite costly and there"] # [doc = "    is perhaps some redundant work being performed depending on the specific"] # [doc = "    states in the set. For example, we might be able to only visit some"] # [doc = "    elements of the alphabet based on the transitions."] # [doc = " 3. Move parts of minimization into determinization. If minimization has"] # [doc = "    fewer states to deal with, then it should run faster. A prime example"] # [doc = "    of this might be large Unicode classes, which are generated in way that"] # [doc = "    can create a lot of redundant states. (Some work has been done on this"] # [doc = "    point during NFA compilation via the algorithm described in the"] # [doc = "    \"Incremental Construction of MinimalAcyclic Finite-State Automata\""] # [doc = "    paper.)"] pub (crate) struct Minimizer < 'a > { dfa : & 'a mut dense :: OwnedDFA , in_transitions : Vec < Vec < Vec < StateID > > > , partitions : Vec < StateSet > , waiting : Vec < StateSet > , }
    };
}

Minimizer!()