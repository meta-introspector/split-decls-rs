macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! Header {
    () => {
        deps!();
        # [doc = " TZif header"] # [derive (Debug)] struct Header { # [doc = " TZif version"] version : Version , # [doc = " Number of UT/local indicators"] ut_local_count : usize , # [doc = " Number of standard/wall indicators"] std_wall_count : usize , # [doc = " Number of leap-second records"] leap_count : usize , # [doc = " Number of transition times"] transition_count : usize , # [doc = " Number of local time type records"] type_count : usize , # [doc = " Number of time zone names bytes"] char_count : usize , }
    };
}

Header!();