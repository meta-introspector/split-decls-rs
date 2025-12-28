macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! value_continuation {
    () => {
        deps!();
        mod value_continuation { use bstr :: ByteSlice ; use winnow :: error :: InputError ; use crate :: parse :: { tests :: util :: { newline_custom_event , newline_event , value_done_event , value_not_done_event } , Event , } ; pub fn value_impl < 'a > (mut i : & 'a [u8] , events : & mut Vec < Event < 'a > > ,) -> winnow :: ModalResult < (& 'a [u8] , ()) , InputError < & 'a [u8] > > { super :: value_impl (& mut i , & mut | e | events . push (e)) . map (| _ | (i , ())) } # [test] fn simple_continuation () { let mut events = Vec :: new () ; assert_eq ! (value_impl (b"hello\\\nworld" , & mut events) . unwrap () . 0 , b"") ; assert_eq ! (events , vec ! [value_not_done_event ("hello") , newline_event () , value_done_event ("world")]) ; } # [test] fn continuation_with_whitespace () { let mut events = Vec :: new () ; assert_eq ! (value_impl (b"hello\\\n        world" , & mut events) . unwrap () . 0 , b"") ; assert_eq ! (events , vec ! [value_not_done_event ("hello") , newline_event () , value_done_event ("        world")]) ; let mut events = Vec :: new () ; assert_eq ! (value_impl (b"hello\\\r\n        world" , & mut events) . unwrap () . 0 , b"") ; assert_eq ! (events , vec ! [value_not_done_event ("hello") , newline_custom_event ("\r\n") , value_done_event ("        world")]) ; let mut events = Vec :: new () ; assert ! (value_impl (b"hello\\\r\r\n        world" , & mut events) . is_err () , r"\r must be followed by \n") ; } # [test] fn complex_continuation_with_leftover_comment () { let mut events = Vec :: new () ; assert_eq ! (value_impl (b"1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c" , & mut events) . unwrap () . 0 , b" # \"b\t ; c") ; assert_eq ! (events , vec ! [value_not_done_event (r#"1    "\""#) , newline_event () , value_not_done_event (r#"a ; e "\""#) , newline_event () , value_done_event ("d")]) ; } # [test] fn quote_split_over_two_lines_with_leftover_comment () { let mut events = Vec :: new () ; assert_eq ! (value_impl (b"\"\\\n;\";a" , & mut events) . unwrap () . 0 , b";a") ; assert_eq ! (events , vec ! [value_not_done_event ("\"") , newline_event () , value_done_event (";\"")]) ; let mut events = Vec :: new () ; assert_eq ! (value_impl (b"\"a\\\r\nb;\";c" , & mut events) . unwrap () . 0 , b";c") ; assert_eq ! (events , vec ! [value_not_done_event ("\"a") , newline_custom_event ("\r\n") , value_done_event ("b;\"")]) ; } # [test] fn quote_split_over_multiple_lines_without_surrounding_quotes_but_inner_quotes () { let mut events = Vec :: new () ; assert_eq ! (value_impl (br#"1\
"2" a\
\"3 b\"\
4 ; comment "# , & mut events) . unwrap () . 0 . as_bstr () , b" ; comment " . as_bstr ()) ; assert_eq ! (events , vec ! [value_not_done_event ("1") , newline_event () , value_not_done_event (r#""2" a"#) , newline_event () , value_not_done_event (r#"\"3 b\""#) , newline_event () , value_done_event ("4")]) ; } # [test] fn quote_split_over_multiple_lines_with_surrounding_quotes () { let mut events = Vec :: new () ; assert_eq ! (value_impl (br#""1\
"2" a\
\"3 b\"\
4 " ; comment "# , & mut events) . unwrap () . 0 . as_bstr () , b" ; comment " . as_bstr ()) ; assert_eq ! (events , vec ! [value_not_done_event ("\"1") , newline_event () , value_not_done_event (r#""2" a"#) , newline_event () , value_not_done_event (r#"\"3 b\""#) , newline_event () , value_done_event ("4 \"")]) ; } }
    };
}

value_continuation!()