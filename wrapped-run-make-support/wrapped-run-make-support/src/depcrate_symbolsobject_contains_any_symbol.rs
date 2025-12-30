// Generated macro for object_contains_any_symbol (function)
macro_rules! Depcrate_symbolsobject_contains_any_symbol {
() => {
// Module: crate::symbols
// Provides: {"object_contains_any_symbol"}
// Dependencies: {}
# [doc = " Check an object file's symbols for any exact matches against those provided in"] # [doc = " `candidate_symbols`."] # [doc = ""] # [doc = " Returns `true` if **any** of the symbols found in the object file at `path` contain an **exact"] # [doc = " match** against those listed in `candidate_symbols`. Take care to account for (1) platform"] # [doc = " differences and (2) calling convention and symbol decorations differences."] # [doc = ""] # [doc = " Panics if `path` is not a valid object file readable by the current user or if `path` cannot be"] # [doc = " parsed as a recognized object file."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " See [`object_contains_any_symbol_substring`]."] # [track_caller] pub fn object_contains_any_symbol < P , S > (path : P , candidate_symbols : & [S]) -> bool where P : AsRef < Path > , S : AsRef < str > , { let path = path . as_ref () ; let blob = crate :: fs :: read (path) ; let obj = object :: File :: parse (& * blob) . unwrap_or_else (| e | panic ! ("failed to parse `{}`: {e}" , path . display ())) ; let candidate_symbols = candidate_symbols . iter () . map (| s | s . as_ref ()) . collect :: < Vec < _ > > () ; for sym in obj . symbols () { for candidate_symbol in & candidate_symbols { if sym . name_bytes () . unwrap () == candidate_symbol . as_bytes () { return true ; } } } false }
};
}
