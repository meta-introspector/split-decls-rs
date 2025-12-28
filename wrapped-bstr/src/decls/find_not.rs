macro_rules! find_not {
    () => {
        # [inline] pub (crate) fn find_not (haystack : & [u8] , byteset : & [u8]) -> Option < usize > { if haystack . is_empty () { return None ; } match byteset . len () { 0 => Some (0) , 1 => scalar :: inv_memchr (byteset [0] , haystack) , 2 => scalar :: forward_search_bytes (haystack , | b | { b != byteset [0] && b != byteset [1] }) , 3 => scalar :: forward_search_bytes (haystack , | b | { b != byteset [0] && b != byteset [1] && b != byteset [2] }) , _ => { let table = build_table (byteset) ; scalar :: forward_search_bytes (haystack , | b | table [b as usize] == 0) } } }
    };
}

find_not!();