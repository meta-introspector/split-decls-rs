// Generated macro for Token (enum)
macro_rules! Depcrate_tree_builder_typesToken {
() => {
// Module: crate::tree_builder::types
// Provides: {"Token"}
// Dependencies: {}
# [doc = " A subset/refinement of `tokenizer::Token`.  Everything else is handled"] # [doc = " specially at the beginning of `process_token`."] # [derive (PartialEq , Eq , Clone , Debug)] pub enum Token { TagToken (Tag) , CommentToken (StrTendril) , CharacterTokens (SplitStatus , StrTendril) , NullCharacterToken , EOFToken , }
};
}
