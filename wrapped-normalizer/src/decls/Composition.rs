macro_rules! deps {
    () => {
        Decomposition!();
    };
}

macro_rules! Composition {
    () => {
        deps!();
        # [doc = " An iterator adaptor that turns an `Iterator` over `char` into"] # [doc = " a lazily-decomposed and then canonically composed `char` sequence."] # [derive (Debug)] pub struct Composition < 'data , I > where I : Iterator < Item = char > , { # [doc = " The decomposing part of the normalizer than operates before"] # [doc = " the canonical composition is performed on its output."] decomposition : Decomposition < 'data , I > , # [doc = " Non-Hangul canonical composition data."] canonical_compositions : Char16Trie < 'data > , # [doc = " To make `next()` yield in cases where there's a non-composing"] # [doc = " starter in the decomposition buffer, we put it here to let it"] # [doc = " wait for the next `next()` call (or a jump forward within the"] # [doc = " `next()` call)."] unprocessed_starter : Option < char > , # [doc = " The lowest character for which any one of the following does"] # [doc = " not hold:"] # [doc = " 1. Roundtrips via decomposition and recomposition."] # [doc = " 2. Decomposition starts with a non-starter"] # [doc = " 3. Is not a backward-combining starter"] composition_passthrough_bound : u32 , }
    };
}

Composition!()