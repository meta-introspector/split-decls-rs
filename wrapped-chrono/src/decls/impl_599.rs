macro_rules! deps {
    () => {
        Cursor!();
        Version!();
        State!();
        Error!();
        Header!();
    };
}

macro_rules! impl_599 {
    () => {
        deps!();
        impl < 'a > State < 'a > { # [doc = " Read TZif data blocks"] fn new (cursor : & mut Cursor < 'a > , first : bool) -> Result < Self , Error > { let header = Header :: new (cursor) ? ; let time_size = match first { true => 4 , false => 8 , } ; Ok (Self { time_size , transition_times : cursor . read_exact (header . transition_count * time_size) ? , transition_types : cursor . read_exact (header . transition_count) ? , local_time_types : cursor . read_exact (header . type_count * 6) ? , names : cursor . read_exact (header . char_count) ? , leap_seconds : cursor . read_exact (header . leap_count * (time_size + 4)) ? , std_walls : cursor . read_exact (header . std_wall_count) ? , ut_locals : cursor . read_exact (header . ut_local_count) ? , header , }) } # [doc = " Parse time values"] fn parse_time (& self , arr : & [u8] , version : Version) -> Result < i64 , Error > { match version { Version :: V1 => Ok (read_be_i32 (& arr [.. 4]) ? . into ()) , Version :: V2 | Version :: V3 => read_be_i64 (arr) , } } }
    };
}

impl_599!();