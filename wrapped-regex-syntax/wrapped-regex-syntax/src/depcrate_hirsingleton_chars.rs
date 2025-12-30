// Generated macro for singleton_chars (function)
macro_rules! Depcrate_hirsingleton_chars {
() => {
// Module: crate::hir
// Provides: {"singleton_chars"}
// Dependencies: {}
# [doc = " Given a sequence of HIR values where each value corresponds to a literal"] # [doc = " that is a single `char`, return that sequence of `char`s. Otherwise return"] # [doc = " None. No deduplication is done."] fn singleton_chars (hirs : & [Hir]) -> Option < Vec < char > > { let mut singletons = vec ! [] ; for hir in hirs . iter () { let literal = match * hir . kind () { HirKind :: Literal (Literal (ref bytes)) => bytes , _ => return None , } ; let ch = match crate :: debug :: utf8_decode (literal) { None => return None , Some (Err (_)) => return None , Some (Ok (ch)) => ch , } ; if literal . len () != ch . len_utf8 () { return None ; } singletons . push (ch) ; } Some (singletons) }
};
}
