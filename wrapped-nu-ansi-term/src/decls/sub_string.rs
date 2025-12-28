macro_rules! deps {
    () => {
        AnsiString!();
        AnsiStrings!();
    };
}

macro_rules! sub_string {
    () => {
        deps!();
        # [doc = " Return a substring of the given AnsiStrings sequence, while keeping the formatting."] pub fn sub_string (start : usize , len : usize , strs : & AnsiStrings) -> Vec < AnsiString < 'static > > { let mut vec = Vec :: new () ; let mut pos = start ; let mut len_rem = len ; for i in strs . 0 . iter () { let frag_len = i . string . len () ; if pos >= frag_len { pos -= frag_len ; continue ; } if len_rem == 0 { break ; } let end = pos + len_rem ; let pos_end = if end >= frag_len { frag_len } else { end } ; vec . push (i . style_ref () . paint (String :: from (& i . string [pos .. pos_end]))) ; if end <= frag_len { break ; } len_rem -= pos_end - pos ; pos = 0 ; } vec }
    };
}

sub_string!();