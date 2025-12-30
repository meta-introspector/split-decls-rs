// Generated macro for split_rules (function)
macro_rules! Depcratesplit_rules {
() => {
// Module: crate
// Provides: {"split_rules"}
// Dependencies: {}
fn split_rules (t : TokenStream) -> Vec < String > { let tts : Vec < _ > = t . into_iter () . collect () ; tts . split (| tt | match tt { TokenTree :: Punct (p) => p . as_char () == ',' , _ => false , }) . filter (| parts | ! parts . is_empty ()) . map (| parts | { parts . into_iter () . map (| part | part . to_string ()) . collect :: < String > () }) . collect () }
};
}
