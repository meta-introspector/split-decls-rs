macro_rules! deps {
    () => {
        NaiveDateTime!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        # [doc = " The `Debug` output of the naive date and time `dt` is the same as"] # [doc = " [`dt.format(\"%Y-%m-%dT%H:%M:%S%.f\")`](crate::format::strftime)."] # [doc = ""] # [doc = " The string printed can be readily parsed via the `parse` method on `str`."] # [doc = ""] # [doc = " It should be noted that, for leap seconds not on the minute boundary,"] # [doc = " it may print a representation not distinguishable from non-leap seconds."] # [doc = " This doesn't matter in practice, since such leap seconds never happened."] # [doc = " (By the time of the first leap second on 1972-06-30,"] # [doc = " every time zone offset around the world has standardized to the 5-minute alignment.)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::NaiveDate;"] # [doc = ""] # [doc = " let dt = NaiveDate::from_ymd_opt(2016, 11, 15).unwrap().and_hms_opt(7, 39, 24).unwrap();"] # [doc = " assert_eq!(format!(\"{:?}\", dt), \"2016-11-15T07:39:24\");"] # [doc = " ```"] # [doc = ""] # [doc = " Leap seconds may also be used."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::NaiveDate;"] # [doc = " let dt ="] # [doc = "     NaiveDate::from_ymd_opt(2015, 6, 30).unwrap().and_hms_milli_opt(23, 59, 59, 1_500).unwrap();"] # [doc = " assert_eq!(format!(\"{:?}\", dt), \"2015-06-30T23:59:60.500\");"] # [doc = " ```"] impl fmt :: Debug for NaiveDateTime { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . date . fmt (f) ? ; f . write_char ('T') ? ; self . time . fmt (f) } }
    };
}

impl_434!();