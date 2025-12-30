// Generated macro for Utf8Compiler (struct)
macro_rules! Depcrate_nfa_thompson_compilerUtf8Compiler {
() => {
// Module: crate::nfa::thompson::compiler
// Provides: {"Utf8Compiler"}
// Dependencies: {}
# [doc = " A UTF-8 compiler based on Daciuk's algorithm for compilining minimal DFAs"] # [doc = " from a lexicographically sorted sequence of strings in linear time."] # [doc = ""] # [doc = " The trick here is that any Unicode codepoint range can be converted to"] # [doc = " a sequence of byte ranges that form a UTF-8 automaton. Connecting them"] # [doc = " together via an alternation is trivial, and indeed, it works. However,"] # [doc = " there is a lot of redundant structure in many UTF-8 automatons. Since our"] # [doc = " UTF-8 ranges are in lexicographic order, we can use Daciuk's algorithm"] # [doc = " to build nearly minimal DFAs in linear time. (They are guaranteed to be"] # [doc = " minimal because we use a bounded cache of previously build DFA states.)"] # [doc = ""] # [doc = " The drawback is that this sadly doesn't work for reverse automata, since"] # [doc = " the ranges are no longer in lexicographic order. For that, we invented the"] # [doc = " range trie (which gets its own module). Once a range trie is built, we then"] # [doc = " use this same Utf8Compiler to build a reverse UTF-8 automaton."] # [doc = ""] # [doc = " The high level idea is described here:"] # [doc = " https://blog.burntsushi.net/transducers/#finite-state-machines-as-data-structures"] # [doc = ""] # [doc = " There is also another implementation of this in the `fst` crate."] # [derive (Debug)] struct Utf8Compiler < 'a > { builder : & 'a mut Builder , state : & 'a mut Utf8State , target : StateID , }
};
}
