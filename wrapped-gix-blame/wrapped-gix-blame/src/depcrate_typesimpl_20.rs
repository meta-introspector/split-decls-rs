// Generated macro for impl_20 (impl)
macro_rules! Depcrate_typesimpl_20 {
() => {
// Module: crate::types
// Provides: {"impl_20"}
// Dependencies: {}
impl Outcome { # [doc = " Return an iterator over each entry in [`Self::entries`], along with its lines, line by line."] # [doc = ""] # [doc = " Note that [`Self::blob`] must be tokenized in exactly the same way as the tokenizer that was used"] # [doc = " to perform the diffs, which is what this method assures."] pub fn entries_with_lines (& self) -> impl Iterator < Item = (BlameEntry , Vec < BString >) > + '_ { use imara_diff :: TokenSource ; let mut interner = imara_diff :: Interner :: new (self . blob . len () / 100) ; let lines_as_tokens : Vec < _ > = tokens_for_diffing (& self . blob) . tokenize () . map (| token | interner . intern (token)) . collect () ; self . entries . iter () . map (move | e | { (e . clone () , lines_as_tokens [e . range_in_blamed_file ()] . iter () . map (| token | BString :: new (interner [* token] . into ())) . collect () ,) }) } }
};
}
