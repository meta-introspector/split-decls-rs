macro_rules! deps {
    () => {
        OutOfRange!();
        UtcDateTime!();
        Error!();
    };
}

macro_rules! impl_623 {
    () => {
        deps!();
        impl UtcDateTime { # [doc = " Construct a UTC date time from a Unix time in seconds and nanoseconds"] pub (crate) fn from_timespec (unix_time : i64) -> Result < Self , Error > { let seconds = match unix_time . checked_sub (UNIX_OFFSET_SECS) { Some (seconds) => seconds , None => return Err (Error :: OutOfRange ("out of range operation")) , } ; let mut remaining_days = seconds / SECONDS_PER_DAY ; let mut remaining_seconds = seconds % SECONDS_PER_DAY ; if remaining_seconds < 0 { remaining_seconds += SECONDS_PER_DAY ; remaining_days -= 1 ; } let mut cycles_400_years = remaining_days / DAYS_PER_400_YEARS ; remaining_days %= DAYS_PER_400_YEARS ; if remaining_days < 0 { remaining_days += DAYS_PER_400_YEARS ; cycles_400_years -= 1 ; } let cycles_100_years = Ord :: min (remaining_days / DAYS_PER_100_YEARS , 3) ; remaining_days -= cycles_100_years * DAYS_PER_100_YEARS ; let cycles_4_years = Ord :: min (remaining_days / DAYS_PER_4_YEARS , 24) ; remaining_days -= cycles_4_years * DAYS_PER_4_YEARS ; let remaining_years = Ord :: min (remaining_days / DAYS_PER_NORMAL_YEAR , 3) ; remaining_days -= remaining_years * DAYS_PER_NORMAL_YEAR ; let mut year = OFFSET_YEAR + remaining_years + cycles_4_years * 4 + cycles_100_years * 100 + cycles_400_years * 400 ; let mut month = 0 ; while month < DAY_IN_MONTHS_LEAP_YEAR_FROM_MARCH . len () { let days = DAY_IN_MONTHS_LEAP_YEAR_FROM_MARCH [month] ; if remaining_days < days { break ; } remaining_days -= days ; month += 1 ; } month += 2 ; if month >= MONTHS_PER_YEAR as usize { month -= MONTHS_PER_YEAR as usize ; year += 1 ; } month += 1 ; let month_day = 1 + remaining_days ; let hour = remaining_seconds / SECONDS_PER_HOUR ; let minute = (remaining_seconds / SECONDS_PER_MINUTE) % MINUTES_PER_HOUR ; let second = remaining_seconds % SECONDS_PER_MINUTE ; let year = match year >= i32 :: MIN as i64 && year <= i32 :: MAX as i64 { true => year as i32 , false => return Err (Error :: OutOfRange ("i64 is out of range for i32")) , } ; Ok (Self { year , month : month as u8 , month_day : month_day as u8 , hour : hour as u8 , minute : minute as u8 , second : second as u8 , }) } }
    };
}

impl_623!();