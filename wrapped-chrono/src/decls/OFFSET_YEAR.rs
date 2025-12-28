macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! OFFSET_YEAR {
    () => {
        deps!();
        # [doc = " Offset year"] const OFFSET_YEAR : i64 = 2000 ;
    };
}

OFFSET_YEAR!();