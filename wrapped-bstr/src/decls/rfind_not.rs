macro_rules! rfind_not {
    () => {
        # [inline] pub (crate) fn rfind_not (haystack : & [u8] , byteset : & [u8]) -> Option < usize > { if haystack . is_empty () { return None ; } match byteset . len () { 0 => Some (haystack . len () - 1) , 1 => scalar :: inv_memrchr (byteset [0] , haystack) , 2 => scalar :: reverse_search_bytes (haystack , | b | { b != byteset [0] && b != byteset [1] }) , 3 => scalar :: reverse_search_bytes (haystack , | b | { b != byteset [0] && b != byteset [1] && b != byteset [2] }) , _ => { let table = build_table (byteset) ; scalar :: reverse_search_bytes (haystack , | b | table [b as usize] == 0) } } }
    };
}

rfind_not!()