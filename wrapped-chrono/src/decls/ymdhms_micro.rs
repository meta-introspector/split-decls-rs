macro_rules! deps {
    () => {
        FixedOffset!();
        DateTime!();
    };
}

macro_rules! ymdhms_micro {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] # [cfg (feature = "alloc")] fn ymdhms_micro (fixedoffset : & FixedOffset , year : i32 , month : u32 , day : u32 , hour : u32 , min : u32 , sec : u32 , micro : u32 ,) -> DateTime < FixedOffset > { fixedoffset . with_ymd_and_hms (year , month , day , hour , min , sec) . unwrap () . with_nanosecond (micro * 1000) . unwrap () }
    };
}

ymdhms_micro!()