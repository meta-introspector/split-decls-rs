macro_rules! deps {
    () => {
        Month!();
    };
}

macro_rules! DAY_IN_MONTHS_LEAP_YEAR_FROM_MARCH {
    () => {
        deps!();
        # [doc = " Month days in a leap year from March"] const DAY_IN_MONTHS_LEAP_YEAR_FROM_MARCH : [i64 ; 12] = [31 , 30 , 31 , 30 , 31 , 31 , 30 , 31 , 30 , 31 , 31 , 29] ;
    };
}

DAY_IN_MONTHS_LEAP_YEAR_FROM_MARCH!();