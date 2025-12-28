macro_rules! deps {
    () => {
        FixedOffset!();
        DateTime!();
    };
}

macro_rules! ymdhms {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] fn ymdhms (fixedoffset : & FixedOffset , year : i32 , month : u32 , day : u32 , hour : u32 , min : u32 , sec : u32 ,) -> DateTime < FixedOffset > { fixedoffset . with_ymd_and_hms (year , month , day , hour , min , sec) . unwrap () }
    };
}

ymdhms!()