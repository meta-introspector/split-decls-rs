macro_rules! deps {
    () => {
        Cursor!();
        Error!();
    };
}

macro_rules! parse_rule_time {
    () => {
        deps!();
        # [doc = " Parse transition rule time"] fn parse_rule_time (cursor : & mut Cursor) -> Result < i32 , Error > { let (hour , minute , second) = parse_hhmmss (cursor) ? ; if ! (0 ..= 24) . contains (& hour) { return Err (Error :: InvalidTzString ("invalid day time hour")) ; } if ! (0 ..= 59) . contains (& minute) { return Err (Error :: InvalidTzString ("invalid day time minute")) ; } if ! (0 ..= 59) . contains (& second) { return Err (Error :: InvalidTzString ("invalid day time second")) ; } Ok (hour * 3600 + minute * 60 + second) }
    };
}

parse_rule_time!()