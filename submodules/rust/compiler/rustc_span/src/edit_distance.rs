mkuse!{use std :: { cmp , mem } ;}
mkuse!{use crate :: Symbol ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}

macro_rules! edit_distance_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function edit_distance in module {}", module_path!());
    };
}

mkfn!{
    edit_distance_introspect!();
    # [doc = " Finds the [edit distance] between two strings."] # [doc = ""] # [doc = " Returns `None` if the distance exceeds the limit."] # [doc = ""] # [doc = " [edit distance]: https://en.wikipedia.org/wiki/Edit_distance"] pub fn edit_distance (a : & str , b : & str , limit : usize) -> Option < usize > { let mut a = & a . chars () . collect :: < Vec < _ > > () [..] ; let mut b = & b . chars () . collect :: < Vec < _ > > () [..] ; if a . len () < b . len () { mem :: swap (& mut a , & mut b) ; } let min_dist = a . len () - b . len () ; if min_dist > limit { return None ; } while let Some (((b_char , b_rest) , (a_char , a_rest))) = b . split_first () . zip (a . split_first ()) && a_char == b_char { a = a_rest ; b = b_rest ; } while let Some (((b_char , b_rest) , (a_char , a_rest))) = b . split_last () . zip (a . split_last ()) && a_char == b_char { a = a_rest ; b = b_rest ; } if b . len () == 0 { return Some (min_dist) ; } let mut prev_prev = vec ! [usize :: MAX ; b . len () + 1] ; let mut prev = (0 ..= b . len ()) . collect :: < Vec < _ > > () ; let mut current = vec ! [0 ; b . len () + 1] ; for i in 1 ..= a . len () { current [0] = i ; let a_idx = i - 1 ; for j in 1 ..= b . len () { let b_idx = j - 1 ; let substitution_cost = if a [a_idx] == b [b_idx] { 0 } else { 1 } ; current [j] = cmp :: min (prev [j] + 1 , cmp :: min (current [j - 1] + 1 , prev [j - 1] + substitution_cost ,) ,) ; if (i > 1) && (j > 1) && (a [a_idx] == b [b_idx - 1]) && (a [a_idx - 1] == b [b_idx]) { current [j] = cmp :: min (current [j] , prev_prev [j - 2] + 1) ; } } [prev_prev , prev , current] = [prev , current , prev_prev] ; } let distance = prev [b . len ()] ; (distance <= limit) . then_some (distance) }
}

macro_rules! edit_distance_with_substrings_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function edit_distance_with_substrings in module {}", module_path!());
    };
}

mkfn!{
    edit_distance_with_substrings_introspect!();
    # [doc = " Provides a word similarity score between two words that accounts for substrings being more"] # [doc = " meaningful than a typical edit distance. The lower the score, the closer the match. 0 is an"] # [doc = " identical match."] # [doc = ""] # [doc = " Uses the edit distance between the two strings and removes the cost of the length difference."] # [doc = " If this is 0 then it is either a substring match or a full word match, in the substring match"] # [doc = " case we detect this and return `1`. To prevent finding meaningless substrings, eg. \"in\" in"] # [doc = " \"shrink\", we only perform this subtraction of length difference if one of the words is not"] # [doc = " greater than twice the length of the other. For cases where the words are close in size but not"] # [doc = " an exact substring then the cost of the length difference is discounted by half."] # [doc = ""] # [doc = " Returns `None` if the distance exceeds the limit."] pub fn edit_distance_with_substrings (a : & str , b : & str , limit : usize) -> Option < usize > { let n = a . chars () . count () ; let m = b . chars () . count () ; let big_len_diff = (n * 2) < m || (m * 2) < n ; let len_diff = m . abs_diff (n) ; let distance = edit_distance (a , b , limit + len_diff) ? ; let score = distance - len_diff ; let score = if score == 0 && len_diff > 0 && ! big_len_diff { 1 } else if ! big_len_diff { score + len_diff . div_ceil (2) } else { score + len_diff } ; (score <= limit) . then_some (score) }
}

macro_rules! find_best_match_for_name_with_substrings_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_best_match_for_name_with_substrings in module {}", module_path!());
    };
}

mkfn!{
    find_best_match_for_name_with_substrings_introspect!();
    # [doc = " Finds the best match for given word in the given iterator where substrings are meaningful."] # [doc = ""] # [doc = " A version of [`find_best_match_for_name`] that uses [`edit_distance_with_substrings`] as the"] # [doc = " score for word similarity. This takes an optional distance limit which defaults to one-third of"] # [doc = " the given word."] # [doc = ""] # [doc = " We use case insensitive comparison to improve accuracy on an edge case with a lower(upper)case"] # [doc = " letters mismatch."] pub fn find_best_match_for_name_with_substrings (candidates : & [Symbol] , lookup : Symbol , dist : Option < usize > ,) -> Option < Symbol > { find_best_match_for_name_impl (true , candidates , lookup , dist) }
}

macro_rules! find_best_match_for_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_best_match_for_name in module {}", module_path!());
    };
}

mkfn!{
    find_best_match_for_name_introspect!();
    # [doc = " Finds the best match for a given word in the given iterator."] # [doc = ""] # [doc = " As a loose rule to avoid the obviously incorrect suggestions, it takes"] # [doc = " an optional limit for the maximum allowable edit distance, which defaults"] # [doc = " to one-third of the given word."] # [doc = ""] # [doc = " We use case insensitive comparison to improve accuracy on an edge case with a lower(upper)case"] # [doc = " letters mismatch."] pub fn find_best_match_for_name (candidates : & [Symbol] , lookup : Symbol , dist : Option < usize > ,) -> Option < Symbol > { find_best_match_for_name_impl (false , candidates , lookup , dist) }
}

macro_rules! find_best_match_for_names_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_best_match_for_names in module {}", module_path!());
    };
}

mkfn!{
    find_best_match_for_names_introspect!();
    # [doc = " Find the best match for multiple words"] # [doc = ""] # [doc = " This function is intended for use when the desired match would never be"] # [doc = " returned due to a substring in `lookup` which is superfluous."] # [doc = ""] # [doc = " For example, when looking for the closest lint name to `clippy:missing_docs`,"] # [doc = " we would find `clippy::erasing_op`, despite `missing_docs` existing and being a better suggestion."] # [doc = " `missing_docs` would have a larger edit distance because it does not contain the `clippy` tool prefix."] # [doc = " In order to find `missing_docs`, this function takes multiple lookup strings, computes the best match"] # [doc = " for each and returns the match which had the lowest edit distance. In our example, `clippy:missing_docs` and"] # [doc = " `missing_docs` would be `lookups`, enabling `missing_docs` to be the best match, as desired."] pub fn find_best_match_for_names (candidates : & [Symbol] , lookups : & [Symbol] , dist : Option < usize > ,) -> Option < Symbol > { lookups . iter () . map (| s | (s , find_best_match_for_name_impl (false , candidates , * s , dist))) . filter_map (| (s , r) | r . map (| r | (s , r))) . min_by (| (s1 , r1) , (s2 , r2) | { let d1 = edit_distance (s1 . as_str () , r1 . as_str () , usize :: MAX) . unwrap () ; let d2 = edit_distance (s2 . as_str () , r2 . as_str () , usize :: MAX) . unwrap () ; d1 . cmp (& d2) }) . map (| (_ , r) | r) }
}

macro_rules! find_best_match_for_name_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_best_match_for_name_impl in module {}", module_path!());
    };
}

mkfn!{
    find_best_match_for_name_impl_introspect!();
    # [cold] fn find_best_match_for_name_impl (use_substring_score : bool , candidates : & [Symbol] , lookup_symbol : Symbol , dist : Option < usize > ,) -> Option < Symbol > { let lookup = lookup_symbol . as_str () ; let lookup_uppercase = lookup . to_uppercase () ; if let Some (c) = candidates . iter () . find (| c | c . as_str () . to_uppercase () == lookup_uppercase) { return Some (* c) ; } let lookup_len = lookup . chars () . count () ; let mut dist = dist . unwrap_or_else (| | cmp :: max (lookup_len , 3) / 3) ; let mut best = None ; let mut next_candidates = vec ! [] ; for c in candidates { match if use_substring_score { edit_distance_with_substrings (lookup , c . as_str () , dist) } else { edit_distance (lookup , c . as_str () , dist) } { Some (0) => return Some (* c) , Some (d) => { if use_substring_score { if d < dist { dist = d ; next_candidates . clear () ; } else { } next_candidates . push (* c) ; } else { dist = d - 1 ; } best = Some (* c) ; } None => { } } } if next_candidates . len () > 1 { debug_assert ! (use_substring_score) ; best = find_best_match_for_name_impl (false , & next_candidates , lookup_symbol , Some (lookup . len ()) ,) ; } if best . is_some () { return best ; } find_match_by_sorted_words (candidates , lookup) }
}

macro_rules! find_match_by_sorted_words_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_match_by_sorted_words in module {}", module_path!());
    };
}

mkfn!{
    find_match_by_sorted_words_introspect!();
    fn find_match_by_sorted_words (iter_names : & [Symbol] , lookup : & str) -> Option < Symbol > { let lookup_sorted_by_words = sort_by_words (lookup) ; iter_names . iter () . fold (None , | result , candidate | { if sort_by_words (candidate . as_str ()) == lookup_sorted_by_words { Some (* candidate) } else { result } }) }
}

macro_rules! sort_by_words_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sort_by_words in module {}", module_path!());
    };
}

mkfn!{
    sort_by_words_introspect!();
    fn sort_by_words (name : & str) -> Vec < & str > { let mut split_words : Vec < & str > = name . split ('_') . collect () ; split_words . sort_unstable () ; split_words }
}