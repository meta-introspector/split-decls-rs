macro_rules! deps {
    () => {
        Utc!();
        DateTime!();
    };
}

macro_rules! test_parse_datetime_utc {
    () => {
        deps!();
        # [test] fn test_parse_datetime_utc () { let valid = ["2001-02-03T04:05:06Z" , "2001-02-03T04:05:06+0000" , "2001-02-03T04:05:06-00:00" , "2001-02-03T04:05:06-01:00" , "2012-12-12 12:12:12Z" , "2012-12-12t12:12:12Z" , "2012-12-12T12:12:12Z" , "2012 -12-12T12:12:12Z" , "2012  -12-12T12:12:12Z" , "2012- 12-12T12:12:12Z" , "2012-  12-12T12:12:12Z" , "2012-12-12T 12:12:12Z" , "2012-12-12T12 :12:12Z" , "2012-12-12T12  :12:12Z" , "2012-12-12T12: 12:12Z" , "2012-12-12T12:  12:12Z" , "2012-12-12T12 : 12:12Z" , "2012-12-12T12:12:12Z " , " 2012-12-12T12:12:12Z" , "2015-02-18T23:16:09.153Z" , "2015-2-18T23:16:09.153Z" , "+2015-2-18T23:16:09.153Z" , "-77-02-18T23:16:09Z" , "+82701-05-6T15:9:60.898989898989Z" ,] ; for & s in & valid { eprintln ! ("test_parse_datetime_utc valid {s:?}") ; let d = match s . parse :: < DateTime < Utc > > () { Ok (d) => d , Err (e) => panic ! ("parsing `{s}` has failed: {e}") , } ; let s_ = format ! ("{d:?}") ; let d_ = match s_ . parse :: < DateTime < Utc > > () { Ok (d) => d , Err (e) => { panic ! ("`{s}` is parsed into `{d:?}`, but reparsing that has failed: {e}") } } ; assert ! (d == d_ , "`{s}` is parsed into `{d:?}`, but reparsed result `{d_:?}` does not match") ; } let invalid = ["" , "Z" , "15Z" , "15:8:9Z" , "15-8-9Z" , "Fri, 09 Aug 2013 23:54:35 GMT" , "Sat Jun 30 23:59:60 2012" , "1441497364.649" , "+1441497364.649" , "+1441497364" , "+1441497364Z" , "2014/02/03 04:05:06Z" , "2001-02-03T04:05:0600:00" , "2015-15-15T15:15:15Z" , "2012-12-12T12:12:12x" , "2012-123-12T12:12:12Z" , "2012-12-77T12:12:12Z" , "2012-12-12T26:12:12Z" , "2012-12-12T12:61:12Z" , "2012-12-12T12:12:62Z" , "2012-12-12 T12:12:12Z" , "2012-12-12T12:12:12ZZ" , "+802701-12-12T12:12:12Z" , "+ 2012-12-12T12:12:12Z" , "  +82701  -  05  -  6  T  15  :  9  : 60.898989898989   Z" ,] ; for & s in & invalid { eprintln ! ("test_parse_datetime_utc invalid {s:?}") ; assert ! (s . parse ::< DateTime < Utc >> () . is_err ()) ; } }
    };
}

test_parse_datetime_utc!();