macro_rules! find {
    () => {
        # [inline] pub (crate) fn find (haystack : & [u8] , byteset : & [u8]) -> Option < usize > { match byteset . len () { 0 => None , 1 => memchr (byteset [0] , haystack) , 2 => memchr2 (byteset [0] , byteset [1] , haystack) , 3 => memchr3 (byteset [0] , byteset [1] , byteset [2] , haystack) , _ => { let table = build_table (byteset) ; scalar :: forward_search_bytes (haystack , | b | table [b as usize] != 0) } } }
    };
}

find!();