// Generated macro for NormalizedPat (enum)
macro_rules! Depcrate_matches_match_same_armsNormalizedPat {
() => {
// Module: crate::matches::match_same_arms
// Provides: {"NormalizedPat"}
// Dependencies: {}
# [derive (Clone , Copy)] enum NormalizedPat < 'a > { Wild , Never , Struct (Option < DefId > , & 'a [(Symbol , Self)]) , Tuple (Option < DefId > , & 'a [Self]) , Or (& 'a [Self]) , Path (Option < DefId >) , LitStr (Symbol) , LitBytes (ByteSymbol) , LitInt (u128) , LitBool (bool) , Range (PatRange) , # [doc = " A slice pattern. If the second value is `None`, then this matches an exact size. Otherwise"] # [doc = " the first value contains everything before the `..` wildcard pattern, and the second value"] # [doc = " contains everything afterwards. Note that either side, or both sides, may contain zero"] # [doc = " patterns."] Slice (& 'a [Self] , Option < & 'a [Self] >) , # [doc = " A placeholder for a pattern that wasn't well formed in some way."] Err (ErrorGuaranteed) , }
};
}
