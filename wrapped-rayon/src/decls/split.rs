macro_rules! split {
    () => {
        # [doc = " Try to split a string near the midpoint."] # [inline] fn split (chars : & str) -> Option < (& str , & str) > { let index = find_char_midpoint (chars) ; if index > 0 { Some (chars . split_at (index)) } else { None } }
    };
}

split!()