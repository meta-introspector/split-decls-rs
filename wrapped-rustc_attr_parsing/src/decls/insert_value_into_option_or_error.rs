macro_rules! deps {
    () => {
        MetaItemParser!();
        Stage!();
        AcceptContext!();
    };
}

macro_rules! insert_value_into_option_or_error {
    () => {
        deps!();
        # [doc = " Tries to insert the value of a `key = value` meta item into an option."] # [doc = ""] # [doc = " Emits an error when either the option was already Some, or the arguments weren't of form"] # [doc = " `name = value`"] fn insert_value_into_option_or_error < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , param : & MetaItemParser < '_ > , item : & mut Option < Symbol > , name : Ident ,) -> Option < () > { if item . is_some () { cx . duplicate_key (name . span , name . name) ; None } else if let Some (v) = param . args () . name_value () && let Some (s) = v . value_as_str () { * item = Some (s) ; Some (()) } else { cx . expected_name_value (param . span () , Some (name . name)) ; None } }
    };
}

insert_value_into_option_or_error!();