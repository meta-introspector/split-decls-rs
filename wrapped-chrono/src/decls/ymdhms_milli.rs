macro_rules! deps {
    () => {
        FixedOffset!();
        DateTime!();
    };
}

macro_rules! ymdhms_milli {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] fn ymdhms_milli (fixedoffset : & FixedOffset , year : i32 , month : u32 , day : u32 , hour : u32 , min : u32 , sec : u32 , milli : u32 ,) -> DateTime < FixedOffset > { fixedoffset . with_ymd_and_hms (year , month , day , hour , min , sec) . unwrap () . with_nanosecond (milli * 1_000_000) . unwrap () }
    };
}

ymdhms_milli!();