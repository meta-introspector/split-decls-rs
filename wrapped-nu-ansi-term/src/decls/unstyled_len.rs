macro_rules! deps {
    () => {
        AnsiStrings!();
    };
}

macro_rules! unstyled_len {
    () => {
        deps!();
        # [doc = " Return the unstyled length of AnsiStrings. This is equaivalent to `unstyle(strs).len()`."] pub fn unstyled_len (strs : & AnsiStrings) -> usize { let mut l = 0 ; for i in strs . 0 . iter () { l += i . string . len () ; } l }
    };
}

unstyled_len!();