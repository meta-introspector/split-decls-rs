macro_rules! deps {
    () => {
        Spec!();
    };
}

macro_rules! common_prefix_len {
    () => {
        deps!();
        fn common_prefix_len (patterns : & [gix_glob :: search :: pattern :: Mapping < Spec >]) -> usize { let mut count = 0 ; let len = patterns . iter () . filter (| p | ! p . value . pattern . is_excluded ()) . map (| p | { count += 1 ; if p . value . pattern . signature . contains (MagicSignature :: ICASE) { p . value . pattern . prefix_len } else { p . pattern . first_wildcard_pos . unwrap_or (p . pattern . text . len ()) } }) . min () . unwrap_or_default () ; if len == 0 { return 0 ; } let mut max_len = len ; if count < 2 { return max_len ; } let mut patterns = patterns . iter () . filter (| p | ! p . value . pattern . is_excluded ()) . map (| p | & p . value . pattern . path) ; let base = & patterns . next () . expect ("at least two patterns") ; for path in patterns { for (idx , (a , b)) in base [.. max_len] . iter () . zip (path [.. max_len] . iter ()) . enumerate () { if * a != * b { max_len = idx ; break ; } } } max_len }
    };
}

common_prefix_len!();