macro_rules! deps {
    () => {
        AcceptContext!();
        ArgParser!();
        Stage!();
    };
}

macro_rules! extract_value {
    () => {
        deps!();
        fn extract_value < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , key : Symbol , arg : & ArgParser < '_ > , span : Span , out_val : & mut Option < (Symbol , Span) > , failed : & mut bool ,) { if out_val . is_some () { cx . duplicate_key (span , key) ; * failed = true ; return ; } let Some (val) = arg . name_value () else { cx . expected_single_argument (arg . span () . unwrap_or (span)) ; * failed = true ; return ; } ; let Some (value_sym) = val . value_as_str () else { cx . expected_string_literal (val . value_span , Some (val . value_as_lit ())) ; * failed = true ; return ; } ; * out_val = Some ((value_sym , val . value_span)) ; }
    };
}

extract_value!();