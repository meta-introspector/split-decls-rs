macro_rules! prefix_is_substring {
    () => {
        # [doc = " Check that every prefix of the given byte string is a substring."] pub (crate) fn prefix_is_substring (bs : & [u8] , mut search : impl FnMut (& [u8] , & [u8]) -> Option < Option < usize > > ,) -> bool { for i in 0 .. bs . len () . saturating_sub (1) { let prefix = & bs [.. i] ; let result = match search (bs , prefix) { None => continue , Some (result) => result , } ; if ! result . is_some () { return false ; } } true }
    };
}

prefix_is_substring!();