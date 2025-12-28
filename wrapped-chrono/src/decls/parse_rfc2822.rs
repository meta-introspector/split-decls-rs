macro_rules! deps {
    () => {
        ParseResult!();
        Parsed!();
    };
}

macro_rules! parse_rfc2822 {
    () => {
        deps!();
        fn parse_rfc2822 < 'a > (parsed : & mut Parsed , mut s : & 'a str) -> ParseResult < (& 'a str , ()) > { macro_rules ! try_consume { ($ e : expr) => { { let (s_ , v) = $ e ?; s = s_ ; v } } ; } s = s . trim_start () ; if let Ok ((s_ , weekday)) = scan :: short_weekday (s) { if ! s_ . starts_with (',') { return Err (INVALID) ; } s = & s_ [1 ..] ; parsed . set_weekday (weekday) ? ; } s = s . trim_start () ; parsed . set_day (try_consume ! (scan :: number (s , 1 , 2))) ? ; s = scan :: space (s) ? ; parsed . set_month (1 + i64 :: from (try_consume ! (scan :: short_month0 (s)))) ? ; s = scan :: space (s) ? ; let prevlen = s . len () ; let mut year = try_consume ! (scan :: number (s , 2 , usize :: MAX)) ; let yearlen = prevlen - s . len () ; match (yearlen , year) { (2 , 0 ..= 49) => { year += 2000 ; } (2 , 50 ..= 99) => { year += 1900 ; } (3 , _) => { year += 1900 ; } (_ , _) => { } } parsed . set_year (year) ? ; s = scan :: space (s) ? ; parsed . set_hour (try_consume ! (scan :: number (s , 2 , 2))) ? ; s = scan :: char (s . trim_start () , b':') ? . trim_start () ; parsed . set_minute (try_consume ! (scan :: number (s , 2 , 2))) ? ; if let Ok (s_) = scan :: char (s . trim_start () , b':') { parsed . set_second (try_consume ! (scan :: number (s_ , 2 , 2))) ? ; } s = scan :: space (s) ? ; parsed . set_offset (i64 :: from (try_consume ! (scan :: timezone_offset_2822 (s)))) ? ; while let Ok ((s_out , ())) = scan :: comment_2822 (s) { s = s_out ; } Ok ((s , ())) }
    };
}

parse_rfc2822!();