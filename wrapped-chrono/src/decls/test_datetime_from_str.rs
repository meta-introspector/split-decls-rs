macro_rules! deps {
    () => {
        NaiveDateTime!();
    };
}

macro_rules! test_datetime_from_str {
    () => {
        deps!();
        # [test] fn test_datetime_from_str () { let valid = ["2001-02-03T04:05:06" , "2012-12-12T12:12:12" , "2015-02-18T23:16:09.153" , "2015-2-18T23:16:09.153" , "-77-02-18T23:16:09" , "+82701-05-6T15:9:60.898989898989" , "  +82701  -  05  -  6  T  15  :  9  : 60.898989898989   " ,] ; for & s in & valid { eprintln ! ("test_parse_naivedatetime valid {s:?}") ; let d = match s . parse :: < NaiveDateTime > () { Ok (d) => d , Err (e) => panic ! ("parsing `{s}` has failed: {e}") , } ; let s_ = format ! ("{d:?}") ; let d_ = match s_ . parse :: < NaiveDateTime > () { Ok (d) => d , Err (e) => { panic ! ("`{s}` is parsed into `{d:?}`, but reparsing that has failed: {e}") } } ; assert ! (d == d_ , "`{s}` is parsed into `{d:?}`, but reparsed result \
             `{d_:?}` does not match") ; } let invalid = ["" , "x" , "15" , "15:8:9" , "15-8-9" , "Fri, 09 Aug 2013 23:54:35 GMT" , "Sat Jun 30 23:59:60 2012" , "1441497364.649" , "+1441497364.649" , "+1441497364" , "2014/02/03 04:05:06" , "2015-15-15T15:15:15" , "2012-12-12T12:12:12x" , "2012-12-12T12:12:12+00:00" , "2012-12-12T12:12:12 +00:00" , "2012-12-12T12:12:12 GMT" , "2012-123-12T12:12:12" , "2012-12-12t12:12:12" , "2012-12-12 12:12:12" , "2012-12-12T12:12:12Z" , "+ 82701-123-12T12:12:12" , "+802701-123-12T12:12:12" ,] ; for & s in & invalid { eprintln ! ("test_datetime_from_str invalid {s:?}") ; assert ! (s . parse ::< NaiveDateTime > () . is_err ()) ; } }
    };
}

test_datetime_from_str!()