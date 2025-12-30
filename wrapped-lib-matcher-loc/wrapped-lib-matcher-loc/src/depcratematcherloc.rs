// Generated macro for MatcherLoc (enum)
macro_rules! DepcrateMatcherLoc {
() => {
// Module: crate
// Provides: {"MatcherLoc"}
// Dependencies: {}
# [doc = " A unit within a matcher that a `MatcherPos` can refer to. Similar to (and derived from)"] # [doc = " `mbe::TokenTree`, but designed specifically for fast and easy traversal during matching."] # [doc = " Notable differences to `mbe::TokenTree`:"] # [doc = " - It is non-recursive, i.e. there is no nesting."] # [doc = " - The end pieces of each sequence (the separator, if present, and the Kleene op) are"] # [doc = "   represented explicitly, as is the very end of the matcher."] # [doc = ""] # [doc = " This means a matcher can be represented by `&[MatcherLoc]`, and traversal mostly involves"] # [doc = " simply incrementing the current matcher position index by one."] # [derive (Debug , PartialEq , Clone)] pub enum MatcherLoc { Token { token : Token , } , Delimited , Sequence { op : KleeneOp , num_metavar_decls : usize , idx_first_after : usize , next_metavar : usize , seq_depth : usize , } , SequenceKleeneOpNoSep { op : KleeneOp , idx_first : usize , } , SequenceSep { separator : Token , } , SequenceKleeneOpAfterSep { idx_first : usize , } , MetaVarDecl { span : Span , bind : Ident , kind : NonterminalKind , next_metavar : usize , seq_depth : usize , } , Eof , }
};
}
