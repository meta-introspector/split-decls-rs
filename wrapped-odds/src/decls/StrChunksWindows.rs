macro_rules! deps {
    () => {
        CharChunks!();
        CharWindows!();
    };
}

macro_rules! StrChunksWindows {
    () => {
        deps!();
        # [doc = " Extension traits for the `char_chunks` and `char_windows` methods"] pub trait StrChunksWindows { # [doc = " Return an iterator that splits the string in substrings of each `n`"] # [doc = " `char` per substring. The last item will contain the remainder if"] # [doc = " `n` does not divide the char length of the string evenly."] fn char_chunks (& self , n : usize) -> CharChunks ; # [doc = " Return an iterator that produces substrings of each `n`"] # [doc = " `char` per substring in a sliding window that advances one char at a time."] # [doc = ""] # [doc = " ***Panics*** if `n` is zero."] fn char_windows (& self , n : usize) -> CharWindows ; }
    };
}

StrChunksWindows!()