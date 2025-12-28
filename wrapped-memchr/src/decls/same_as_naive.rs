macro_rules! same_as_naive {
    () => {
        # [doc = " Check that naive substring search matches the result of the given search"] # [doc = " algorithm."] pub (crate) fn same_as_naive (reverse : bool , haystack : & [u8] , needle : & [u8] , mut search : impl FnMut (& [u8] , & [u8]) -> Option < Option < usize > > ,) -> bool { let result = match search (haystack , needle) { None => return true , Some (result) => result , } ; if reverse { result == naive :: rfind (haystack , needle) } else { result == naive :: find (haystack , needle) } }
    };
}

same_as_naive!()