macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! SplitN {
    () => {
        deps!();
        # [doc = " An iterator over at most `n` substrings in a byte string, split by a"] # [doc = " separator."] # [doc = ""] # [doc = " `'h` is the lifetime of the byte string being split (the haystack), while"] # [doc = " `'s` is the lifetime of the byte string doing the splitting."] # [derive (Clone , Debug)] pub struct SplitN < 'h , 's > { split : Split < 'h , 's > , limit : usize , count : usize , }
    };
}

SplitN!();