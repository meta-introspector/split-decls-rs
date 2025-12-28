macro_rules! deps {
    () => {
        Hunk!();
        ConflictStyle!();
    };
}

macro_rules! hunks_differ_in_diff3 {
    () => {
        deps!();
        # [doc = " Used only when `diff3` is the conflict style as `zdiff3` automatically reduces hunks into nothing."] # [doc = " Here we check if all hunks are the same."] pub fn hunks_differ_in_diff3 (style : ConflictStyle , a : & [Hunk] , b : & [Hunk] , input : & InternedInput < & [u8] > , current_tokens : & [Token] ,) -> bool { if style != ConflictStyle :: Diff3 { return true ; } let tokens_for_hunk = | hunk : & Hunk | -> & [Token] { & tokens_for_side (hunk . side , input , current_tokens) [hunk . after . start as usize .. hunk . after . end as usize] } ; a . iter () . flat_map (tokens_for_hunk) . ne (b . iter () . flat_map (tokens_for_hunk)) }
    };
}

hunks_differ_in_diff3!()