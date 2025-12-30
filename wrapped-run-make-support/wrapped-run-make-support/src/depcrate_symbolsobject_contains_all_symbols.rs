// Generated macro for object_contains_all_symbols (function)
macro_rules! Depcrate_symbolsobject_contains_all_symbols {
() => {
// Module: crate::symbols
// Provides: {"object_contains_all_symbols"}
// Dependencies: {}
# [doc = " Check an object file contains all symbols provided in `candidate_symbols`."] # [doc = ""] # [doc = " Returns `true` if **all** of the symbols in `candidate_symbols` are found within the object file"] # [doc = " at `path` by **exact match**. Take care to account for (1) platform differences and (2) calling"] # [doc = " convention and symbol decorations differences."] # [doc = ""] # [doc = " Panics if `path` is not a valid object file readable by the current user or if `path` cannot be"] # [doc = " parsed as a recognized object file."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " See [`object_contains_any_symbol_substring`]."] # [track_caller] pub fn object_contains_all_symbols < P , S > (path : P , candidate_symbols : & [S] ,) -> ContainsAllSymbolsOutcome < '_ > where P : AsRef < Path > , S : AsRef < str > , { let path = path . as_ref () ; let blob = crate :: fs :: read (path) ; let obj = object :: File :: parse (& * blob) . unwrap_or_else (| e | panic ! ("failed to parse `{}`: {e}" , path . display ())) ; let candidate_symbols = candidate_symbols . iter () . map (| s | s . as_ref ()) ; let mut unmatched_symbols = BTreeSet :: from_iter (candidate_symbols) ; unmatched_symbols . retain (| unmatched_symbol | { for sym in obj . symbols () { if sym . name_bytes () . unwrap () == unmatched_symbol . as_bytes () { return false ; } } true }) ; if unmatched_symbols . is_empty () { ContainsAllSymbolsOutcome :: Ok } else { ContainsAllSymbolsOutcome :: MissingSymbols (unmatched_symbols) } }
};
}
