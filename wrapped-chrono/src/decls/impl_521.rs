macro_rules! deps {
    () => {
        NaiveTime!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        # [doc = " The `Display` output of the naive time `t` is the same as"] # [doc = " [`t.format(\"%H:%M:%S%.f\")`](crate::format::strftime)."] # [doc = ""] # [doc = " The string printed can be readily parsed via the `parse` method on `str`."] # [doc = ""] # [doc = " It should be noted that, for leap seconds not on the minute boundary,"] # [doc = " it may print a representation not distinguishable from non-leap seconds."] # [doc = " This doesn't matter in practice, since such leap seconds never happened."] # [doc = " (By the time of the first leap second on 1972-06-30,"] # [doc = " every time zone offset around the world has standardized to the 5-minute alignment.)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::NaiveTime;"] # [doc = ""] # [doc = " assert_eq!(format!(\"{}\", NaiveTime::from_hms_opt(23, 56, 4).unwrap()), \"23:56:04\");"] # [doc = " assert_eq!("] # [doc = "     format!(\"{}\", NaiveTime::from_hms_milli_opt(23, 56, 4, 12).unwrap()),"] # [doc = "     \"23:56:04.012\""] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     format!(\"{}\", NaiveTime::from_hms_micro_opt(23, 56, 4, 1234).unwrap()),"] # [doc = "     \"23:56:04.001234\""] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     format!(\"{}\", NaiveTime::from_hms_nano_opt(23, 56, 4, 123456).unwrap()),"] # [doc = "     \"23:56:04.000123456\""] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " Leap seconds may also be used."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::NaiveTime;"] # [doc = " assert_eq!("] # [doc = "     format!(\"{}\", NaiveTime::from_hms_milli_opt(6, 59, 59, 1_500).unwrap()),"] # [doc = "     \"06:59:60.500\""] # [doc = " );"] # [doc = " ```"] impl fmt :: Display for NaiveTime { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (self , f) } }
    };
}

impl_521!()