macro_rules! deps {
    () => {
        Parsed!();
        Weekday!();
        ParseResult!();
    };
}

macro_rules! set_weekday_with_number_from_monday {
    () => {
        deps!();
        fn set_weekday_with_number_from_monday (p : & mut Parsed , v : i64) -> ParseResult < () > { p . set_weekday (match v { 1 => Weekday :: Mon , 2 => Weekday :: Tue , 3 => Weekday :: Wed , 4 => Weekday :: Thu , 5 => Weekday :: Fri , 6 => Weekday :: Sat , 7 => Weekday :: Sun , _ => return Err (OUT_OF_RANGE) , }) }
    };
}

set_weekday_with_number_from_monday!();