macro_rules! all_whitespace {
    () => {
        # [doc = " Returns `None` if the first `col` chars of `s` contain a non-whitespace char."] # [doc = " Otherwise returns `Some(k)` where `k` is first char offset after that leading"] # [doc = " whitespace. Note that `k` may be outside bounds of `s`."] fn all_whitespace (s : & str , col : CharPos) -> Option < usize > { let mut idx = 0 ; for (i , ch) in s . char_indices () . take (col . to_usize ()) { if ! ch . is_whitespace () { return None ; } idx = i + ch . len_utf8 () ; } Some (idx) }
    };
}

all_whitespace!();