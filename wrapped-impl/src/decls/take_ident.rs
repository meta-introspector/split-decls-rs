macro_rules! take_ident {
    () => {
        fn take_ident < 'a > (read : & mut & 'a str) -> & 'a str { let mut ident_len = 0 ; for ch in read . chars () { match ch { 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => ident_len += 1 , _ => break , } } let (ident , rest) = read . split_at (ident_len) ; * read = rest ; ident }
    };
}

take_ident!();