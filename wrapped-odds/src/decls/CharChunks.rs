macro_rules! CharChunks {
    () => {
        # [doc = " An iterator that splits the string in substrings of each `n`"] # [doc = " `char` per substring. The last item will contain the remainder if"] # [doc = " `n` does not divide the char length of the string evenly."] # [derive (Clone , Debug)] pub struct CharChunks < 'a > { s : & 'a str , n : usize , }
    };
}

CharChunks!();