// Generated macro for LexicalPermutation (trait)
macro_rules! Depcrate_lexicalLexicalPermutation {
() => {
// Module: crate::lexical
// Provides: {"LexicalPermutation"}
// Dependencies: {}
# [doc = " Permute a slice into its next or previous permutation (in lexical order)."] # [doc = ""] # [doc = " ```"] # [doc = " use permutohedron::LexicalPermutation;"] # [doc = ""] # [doc = " let mut data = [1, 2, 3];"] # [doc = " let mut permutations = Vec::new();"] # [doc = ""] # [doc = " loop {"] # [doc = "     permutations.push(data.to_vec());"] # [doc = "     if !data.next_permutation() {"] # [doc = "         break;"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     permutations,"] # [doc = "     &["] # [doc = "         &[1, 2, 3],"] # [doc = "         &[1, 3, 2],"] # [doc = "         &[2, 1, 3],"] # [doc = "         &[2, 3, 1],"] # [doc = "         &[3, 1, 2],"] # [doc = "         &[3, 2, 1]"] # [doc = "     ]"] # [doc = " );"] # [doc = ""] # [doc = " // `data` has been mutated in-place:"] # [doc = " assert_eq!(data, [3, 2, 1]);"] # [doc = " ```"] # [allow (clippy :: module_name_repetitions)] pub trait LexicalPermutation { # [doc = " Return `true` if the slice was permuted, `false` if it is already"] # [doc = " at the last ordered permutation."] fn next_permutation (& mut self) -> bool ; # [doc = " Return `true` if the slice was permuted, `false` if it is already"] # [doc = " at the first ordered permutation."] fn prev_permutation (& mut self) -> bool ; }
};
}
