// Generated macro for make_suggestion (function)
macro_rules! Depcrate_validation_suggestionmake_suggestion {
() => {
// Module: crate::validation::suggestion
// Provides: {"make_suggestion"}
// Dependencies: {}
pub fn make_suggestion < I , A > (prefix : & str , options : I , input : & str) -> Option < String > where I : IntoIterator < Item = A > , A : AsRef < str > , { let mut selected = Vec :: new () ; let mut distances = HashMap :: new () ; for opt in options { let opt = opt . as_ref () . to_string () ; let distance = levenshtein_distance (input , & opt) ; let threshold = (input . len () / 2) . max ((opt . len () / 2) . max (1)) ; if distance < threshold { selected . push (opt . clone ()) ; distances . insert (opt , distance) ; } } if selected . is_empty () { return None ; } selected . sort_by (| a , b | distances [a] . cmp (& distances [b])) ; let mut suggestion = String :: with_capacity (prefix . len () + selected . iter () . map (| s | s . len () + 5) . sum :: < usize > ()) ; suggestion . push_str (prefix) ; suggestion . push (' ') ; for (i , s) in selected . iter () . enumerate () { if i != 0 { suggestion . push_str (", ") ; } write ! (suggestion , "\"{}\"" , s) . unwrap () ; } suggestion . push ('?') ; Some (suggestion) }
};
}
