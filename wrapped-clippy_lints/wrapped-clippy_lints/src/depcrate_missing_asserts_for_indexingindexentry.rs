// Generated macro for IndexEntry (enum)
macro_rules! Depcrate_missing_asserts_for_indexingIndexEntry {
() => {
// Module: crate::missing_asserts_for_indexing
// Provides: {"IndexEntry"}
// Dependencies: {}
# [derive (Debug)] enum IndexEntry < 'hir > { # [doc = " `assert!` without any indexing (so far)"] StrayAssert { asserted_len : usize , comparison : LengthComparison , assert_span : Span , slice : & 'hir Expr < 'hir > , macro_call : Symbol , } , # [doc = " `assert!` with indexing"] # [doc = ""] # [doc = " We also store the highest index to be able to check"] # [doc = " if the `assert!` asserts the right length."] AssertWithIndex { highest_index : usize , is_first_highest : bool , asserted_len : usize , assert_span : Span , slice : & 'hir Expr < 'hir > , indexes : Vec < Span > , comparison : LengthComparison , macro_call : Symbol , } , # [doc = " Indexing without an `assert!`"] IndexWithoutAssert { highest_index : usize , is_first_highest : bool , indexes : Vec < Span > , slice : & 'hir Expr < 'hir > , } , }
};
}
