macro_rules! deps {
    () => {
        Diff!();
    };
}

macro_rules! diff_with {
    () => {
        deps!();
        # [doc = " Compares every element yielded by both `i` and `j` with the given function in lock-step and"] # [doc = " returns a [`Diff`] which describes how `j` differs from `i`."] # [doc = ""] # [doc = " If the number of elements yielded by `j` is less than the number of elements yielded by `i`,"] # [doc = " the number of `j` elements yielded will be returned along with `i`'s remaining elements as"] # [doc = " `Diff::Shorter`."] # [doc = ""] # [doc = " If the two elements of a step differ, the index of those elements along with the remaining"] # [doc = " elements of both `i` and `j` are returned as `Diff::FirstMismatch`."] # [doc = ""] # [doc = " If `i` becomes exhausted before `j` becomes exhausted, the number of elements in `i` along with"] # [doc = " the remaining `j` elements will be returned as `Diff::Longer`."] pub fn diff_with < I , J , F > (i : I , j : J , mut is_equal : F) -> Option < Diff < I :: IntoIter , J :: IntoIter > > where I : IntoIterator , J : IntoIterator , F : FnMut (& I :: Item , & J :: Item) -> bool , { let mut i = i . into_iter () ; let mut j = j . into_iter () ; let mut idx = 0 ; while let Some (i_elem) = i . next () { match j . next () { None => return Some (Diff :: Shorter (idx , put_back (i) . with_value (i_elem))) , Some (j_elem) => { if ! is_equal (& i_elem , & j_elem) { let remaining_i = put_back (i) . with_value (i_elem) ; let remaining_j = put_back (j) . with_value (j_elem) ; return Some (Diff :: FirstMismatch (idx , remaining_i , remaining_j)) ; } } } idx += 1 ; } j . next () . map (| j_elem | Diff :: Longer (idx , put_back (j) . with_value (j_elem))) }
    };
}

diff_with!();