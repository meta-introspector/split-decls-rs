macro_rules! deps {
    () => {
        Find!();
    };
}

macro_rules! Split {
    () => {
        deps!();
        # [doc = " An iterator over substrings in a byte string, split by a separator."] # [doc = ""] # [doc = " `'h` is the lifetime of the byte string being split (the haystack), while"] # [doc = " `'s` is the lifetime of the byte string doing the splitting."] # [derive (Clone , Debug)] pub struct Split < 'h , 's > { finder : Find < 'h , 's > , # [doc = " The end position of the previous match of our splitter. The element"] # [doc = " we yield corresponds to the substring starting at `last` up to the"] # [doc = " beginning of the next match of the splitter."] last : usize , # [doc = " Only set when iteration is complete. A corner case here is when a"] # [doc = " splitter is matched at the end of the haystack. At that point, we still"] # [doc = " need to yield an empty string following it."] done : bool , }
    };
}

Split!();