macro_rules! deps {
    () => {
        State!();
        TransitionRule!();
        Cursor!();
        Error!();
        TimeZone!();
        LocalTimeType!();
        LeapSecond!();
        Transition!();
        Version!();
    };
}

macro_rules! parse {
    () => {
        deps!();
        pub (super) fn parse (bytes : & [u8]) -> Result < TimeZone , Error > { let mut cursor = Cursor :: new (bytes) ; let state = State :: new (& mut cursor , true) ? ; let (state , footer) = match state . header . version { Version :: V1 => match cursor . is_empty () { true => (state , None) , false => { return Err (Error :: InvalidTzFile ("remaining data after end of TZif v1 data block")) ; } } , Version :: V2 | Version :: V3 => { let state = State :: new (& mut cursor , false) ? ; (state , Some (cursor . remaining ())) } } ; let mut transitions = Vec :: with_capacity (state . header . transition_count) ; for (arr_time , & local_time_type_index) in state . transition_times . chunks_exact (state . time_size) . zip (state . transition_types) { let unix_leap_time = state . parse_time (& arr_time [0 .. state . time_size] , state . header . version) ? ; let local_time_type_index = local_time_type_index as usize ; transitions . push (Transition :: new (unix_leap_time , local_time_type_index)) ; } let mut local_time_types = Vec :: with_capacity (state . header . type_count) ; for arr in state . local_time_types . chunks_exact (6) { let ut_offset = read_be_i32 (& arr [.. 4]) ? ; let is_dst = match arr [4] { 0 => false , 1 => true , _ => return Err (Error :: InvalidTzFile ("invalid DST indicator")) , } ; let char_index = arr [5] as usize ; if char_index >= state . header . char_count { return Err (Error :: InvalidTzFile ("invalid time zone name char index")) ; } let position = match state . names [char_index ..] . iter () . position (| & c | c == b'\0') { Some (position) => position , None => return Err (Error :: InvalidTzFile ("invalid time zone name char index")) , } ; let name = & state . names [char_index .. char_index + position] ; let name = if ! name . is_empty () { Some (name) } else { None } ; local_time_types . push (LocalTimeType :: new (ut_offset , is_dst , name) ?) ; } let mut leap_seconds = Vec :: with_capacity (state . header . leap_count) ; for arr in state . leap_seconds . chunks_exact (state . time_size + 4) { let unix_leap_time = state . parse_time (& arr [0 .. state . time_size] , state . header . version) ? ; let correction = read_be_i32 (& arr [state . time_size .. state . time_size + 4]) ? ; leap_seconds . push (LeapSecond :: new (unix_leap_time , correction)) ; } let std_walls_iter = state . std_walls . iter () . copied () . chain (iter :: repeat (0)) ; let ut_locals_iter = state . ut_locals . iter () . copied () . chain (iter :: repeat (0)) ; if std_walls_iter . zip (ut_locals_iter) . take (state . header . type_count) . any (| pair | pair == (0 , 1)) { return Err (Error :: InvalidTzFile ("invalid couple of standard/wall and UT/local indicators" ,)) ; } let extra_rule = match footer { Some (footer) => { let footer = str :: from_utf8 (footer) ? ; if ! (footer . starts_with ('\n') && footer . ends_with ('\n')) { return Err (Error :: InvalidTzFile ("invalid footer")) ; } let tz_string = footer . trim_matches (| c : char | c . is_ascii_whitespace ()) ; if tz_string . starts_with (':') || tz_string . contains ('\0') { return Err (Error :: InvalidTzFile ("invalid footer")) ; } match tz_string . is_empty () { true => None , false => Some (TransitionRule :: from_tz_string (tz_string . as_bytes () , state . header . version == Version :: V3 ,) ?) , } } None => None , } ; TimeZone :: new (transitions , local_time_types , leap_seconds , extra_rule) }
    };
}

parse!();