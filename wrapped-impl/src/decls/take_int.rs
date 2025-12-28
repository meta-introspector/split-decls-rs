macro_rules! take_int {
    () => {
        fn take_int < 'a > (read : & mut & 'a str) -> & 'a str { let mut int_len = 0 ; for ch in read . chars () { match ch { '0' ..= '9' => int_len += 1 , _ => break , } } let (int , rest) = read . split_at (int_len) ; * read = rest ; int }
    };
}

take_int!()