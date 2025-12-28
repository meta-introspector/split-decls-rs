macro_rules! deps {
    () => {
        IgnorableBehavior!();
        CharacterAndClass!();
        CharacterAndTrieValue!();
        Trie!();
    };
}

macro_rules! Decomposition {
    () => {
        deps!();
        # [doc = " An iterator adaptor that turns an `Iterator` over `char` into"] # [doc = " a lazily-decomposed `char` sequence."] # [derive (Debug)] pub struct Decomposition < 'data , I > where I : Iterator < Item = char > , { delegate : I , buffer : SmallVec < [CharacterAndClass ; 17] > , # [doc = " The index of the next item to be read from `buffer`."] # [doc = " The purpose if this index is to avoid having to move"] # [doc = " the rest upon every read."] buffer_pos : usize , pending : Option < CharacterAndTrieValue > , trie : & 'data Trie < 'data > , scalars16 : & 'data ZeroSlice < u16 > , scalars24 : & 'data ZeroSlice < char > , supplementary_scalars16 : & 'data ZeroSlice < u16 > , supplementary_scalars24 : & 'data ZeroSlice < char > , # [doc = " The lowest character for which either of the following does"] # [doc = " not hold:"] # [doc = " 1. Decomposes to self."] # [doc = " 2. Decomposition starts with a non-starter"] decomposition_passthrough_bound : u32 , ignorable_behavior : IgnorableBehavior , }
    };
}

Decomposition!();