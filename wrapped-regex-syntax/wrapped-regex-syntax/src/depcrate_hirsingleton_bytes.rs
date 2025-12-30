// Generated macro for singleton_bytes (function)
macro_rules! Depcrate_hirsingleton_bytes {
() => {
// Module: crate::hir
// Provides: {"singleton_bytes"}
// Dependencies: {}
# [doc = " Given a sequence of HIR values where each value corresponds to a literal"] # [doc = " that is a single byte, return that sequence of bytes. Otherwise return"] # [doc = " None. No deduplication is done."] fn singleton_bytes (hirs : & [Hir]) -> Option < Vec < u8 > > { let mut singletons = vec ! [] ; for hir in hirs . iter () { let literal = match * hir . kind () { HirKind :: Literal (Literal (ref bytes)) => bytes , _ => return None , } ; if literal . len () != 1 { return None ; } singletons . push (literal [0]) ; } Some (singletons) }
};
}
