macro_rules! unescape {
    () => {
        fn unescape (s : & str) -> Option < Cow < '_ , str > > { let mut buf = String :: new () ; let mut prev_end = 0 ; let mut has_error = false ; unescape :: unescape_str (s , | char_range , unescaped_char | { match (unescaped_char , buf . capacity () == 0) { (Ok (c) , false) => buf . push (c) , (Ok (_) , true) if char_range . len () == 1 && char_range . start == prev_end => { prev_end = char_range . end } (Ok (c) , true) => { buf . reserve_exact (s . len ()) ; buf . push_str (& s [.. prev_end]) ; buf . push (c) ; } (Err (_) , _) => has_error = true , } }) ; match (has_error , buf . capacity () == 0) { (true , _) => None , (false , false) => Some (Cow :: Owned (buf)) , (false , true) => Some (Cow :: Borrowed (s)) , } }
    };
}

unescape!();