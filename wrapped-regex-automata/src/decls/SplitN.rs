macro_rules! deps {
    () => {
        Split!();
        Regex!();
    };
}

macro_rules! SplitN {
    () => {
        deps!();
        # [doc = " Yields at most `N` spans delimited by a regular expression match."] # [doc = ""] # [doc = " The spans correspond to the offsets between matches. The last span will be"] # [doc = " whatever remains after splitting."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'r` represents the lifetime of the `Regex` that produced this iterator."] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = ""] # [doc = " This iterator can be created with the [`Regex::splitn`] method."] # [derive (Debug)] pub struct SplitN < 'r , 'h > { splits : Split < 'r , 'h > , limit : usize , }
    };
}

SplitN!()