macro_rules! deps {
    () => {
        Error!();
        Cursor!();
    };
}

macro_rules! parse_signed_hhmmss {
    () => {
        deps!();
        # [doc = " Parse signed hours, minutes and seconds"] fn parse_signed_hhmmss (cursor : & mut Cursor) -> Result < (i32 , i32 , i32 , i32) , Error > { let mut sign = 1 ; if let Some (& c) = cursor . peek () { if c == b'+' || c == b'-' { cursor . read_exact (1) ? ; if c == b'-' { sign = - 1 ; } } } let (hour , minute , second) = parse_hhmmss (cursor) ? ; Ok ((sign , hour , minute , second)) }
    };
}

parse_signed_hhmmss!();