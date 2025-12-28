macro_rules! deps {
    () => {
        Error!();
        Cursor!();
    };
}

macro_rules! parse_rule_time_extended {
    () => {
        deps!();
        # [doc = " Parse transition rule time with TZ string extensions"] fn parse_rule_time_extended (cursor : & mut Cursor) -> Result < i32 , Error > { let (sign , hour , minute , second) = parse_signed_hhmmss (cursor) ? ; if ! (- 167 ..= 167) . contains (& hour) { return Err (Error :: InvalidTzString ("invalid day time hour")) ; } if ! (0 ..= 59) . contains (& minute) { return Err (Error :: InvalidTzString ("invalid day time minute")) ; } if ! (0 ..= 59) . contains (& second) { return Err (Error :: InvalidTzString ("invalid day time second")) ; } Ok (sign * (hour * 3600 + minute * 60 + second)) }
    };
}

parse_rule_time_extended!();