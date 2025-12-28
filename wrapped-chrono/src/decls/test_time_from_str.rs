macro_rules! deps {
    () => {
        NaiveTime!();
    };
}

macro_rules! test_time_from_str {
    () => {
        deps!();
        # [test] fn test_time_from_str () { let valid = ["0:0:0" , "0:0:0.0000000" , "0:0:0.0000003" , " 4 : 3 : 2.1 " , " 09:08:07 " , " 09:08 " , " 9:8:07 " , "01:02:03" , "4:3:2.1" , "9:8:7" , "09:8:7" , "9:08:7" , "9:8:07" , "09:08:7" , "09:8:07" , "09:08:7" , "9:08:07" , "09:08:07" , "9:8:07.123" , "9:08:7.123" , "09:8:7.123" , "09:08:7.123" , "9:08:07.123" , "09:8:07.123" , "09:08:07.123" , "09:08:07.123" , "09:08:07.1234" , "09:08:07.12345" , "09:08:07.123456" , "09:08:07.1234567" , "09:08:07.12345678" , "09:08:07.123456789" , "09:08:07.1234567891" , "09:08:07.12345678912" , "23:59:60.373929310237" ,] ; for & s in & valid { eprintln ! ("test_time_parse_from_str valid {s:?}") ; let d = match s . parse :: < NaiveTime > () { Ok (d) => d , Err (e) => panic ! ("parsing `{s}` has failed: {e}") , } ; let s_ = format ! ("{d:?}") ; let d_ = match s_ . parse :: < NaiveTime > () { Ok (d) => d , Err (e) => { panic ! ("`{s}` is parsed into `{d:?}`, but reparsing that has failed: {e}") } } ; assert ! (d == d_ , "`{s}` is parsed into `{d:?}`, but reparsed result \
                              `{d_:?}` does not match") ; } let invalid = ["" , "x" , "15" , "15:8:" , "15:8:x" , "15:8:9x" , "23:59:61" , "23:54:35 GMT" , "23:54:35 +0000" , "1441497364.649" , "+1441497364.649" , "+1441497364" , "001:02:03" , "01:002:03" , "01:02:003" , "12:34:56.x" , "12:34:56. 0" , "09:08:00000000007" ,] ; for & s in & invalid { eprintln ! ("test_time_parse_from_str invalid {s:?}") ; assert ! (s . parse ::< NaiveTime > () . is_err ()) ; } }
    };
}

test_time_from_str!();