macro_rules! deps {
    () => {
        ArgParser!();
        Stage!();
        AcceptContext!();
    };
}

macro_rules! get {
    () => {
        deps!();
        fn get < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , name : Symbol , param_span : Span , arg : & ArgParser < '_ > , item : & Option < Symbol > ,) -> Option < Symbol > { if item . is_some () { cx . duplicate_key (param_span , name) ; return None ; } if let Some (v) = arg . name_value () { if let Some (value_str) = v . value_as_str () { Some (value_str) } else { cx . expected_string_literal (v . value_span , Some (& v . value_as_lit ())) ; None } } else { cx . expected_name_value (param_span , Some (name)) ; None } }
    };
}

get!();