macro_rules! deps {
    () => {
        EmojiSetDataBorrowed!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl EmojiSetDataBorrowed < '_ > { # [doc = " Check if the set contains the string. Strings consisting of one character"] # [doc = " are treated as a character/code point."] # [doc = ""] # [doc = " This matches ICU behavior for ICU's `UnicodeSet`."] # [inline] pub fn contains_str (self , s : & str) -> bool { self . set . contains_str (s) } # [doc = " Check if the set contains the code point."] # [inline] pub fn contains (self , ch : char) -> bool { self . set . contains (ch) } # [doc = " See [`Self::contains`]."] # [inline] pub fn contains32 (self , cp : u32) -> bool { self . set . contains32 (cp) } }
    };
}

impl_22!()