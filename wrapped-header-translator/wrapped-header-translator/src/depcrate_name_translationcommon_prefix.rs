// Generated macro for common_prefix (function)
macro_rules! Depcrate_name_translationcommon_prefix {
() => {
// Module: crate::name_translation
// Provides: {"common_prefix"}
// Dependencies: {}
# [doc = " Find the common prefix of a number of names, based on word boundaries."] fn common_prefix < 'a > (items : impl IntoIterator < Item = & 'a str >) -> & 'a str { let mut items = items . into_iter () ; let Some (first) = items . next () else { return "" ; } ; let mut min = first ; let mut max = first ; for item in items { if item < min { min = item ; } if max < item { max = item ; } } let mut min_it = Iter { remaining : min } ; let mut max_it = split_words (max) ; while let Some (min_word) = min_it . next () { if let Some (max_word) = max_it . next () { if min_word != max_word { return min . split_at (min . len () - min_it . remaining . len () - min_word . len ()) . 0 ; } } } min }
};
}
