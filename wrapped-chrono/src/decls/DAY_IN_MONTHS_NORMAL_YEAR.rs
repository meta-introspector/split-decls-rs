macro_rules! deps {
    () => {
        Month!();
    };
}

macro_rules! DAY_IN_MONTHS_NORMAL_YEAR {
    () => {
        deps!();
        # [doc = " Month days in a normal year"] const DAY_IN_MONTHS_NORMAL_YEAR : [i64 ; 12] = [31 , 28 , 31 , 30 , 31 , 30 , 31 , 31 , 30 , 31 , 30 , 31] ;
    };
}

DAY_IN_MONTHS_NORMAL_YEAR!();