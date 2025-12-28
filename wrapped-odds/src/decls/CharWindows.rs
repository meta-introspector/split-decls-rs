macro_rules! CharWindows {
    () => {
        # [doc = " An iterator that produces substrings of each `n`"] # [doc = " `char` per substring in a sliding window that advances one char at a time."] # [derive (Clone , Debug)] pub struct CharWindows < 'a > { s : & 'a str , a : usize , b : usize , }
    };
}

CharWindows!()