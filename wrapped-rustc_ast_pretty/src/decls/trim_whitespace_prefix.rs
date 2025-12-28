macro_rules! trim_whitespace_prefix {
    () => {
        fn trim_whitespace_prefix (s : & str , col : CharPos) -> & str { let len = s . len () ; match all_whitespace (s , col) { Some (col) => { if col < len { & s [col ..] } else { "" } } None => s , } }
    };
}

trim_whitespace_prefix!();