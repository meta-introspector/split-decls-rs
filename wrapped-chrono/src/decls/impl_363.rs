macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        # [doc = " The `Display` output of the naive date `d` is the same as"] # [doc = " [`d.format(\"%Y-%m-%d\")`](crate::format::strftime)."] # [doc = ""] # [doc = " The string printed can be readily parsed via the `parse` method on `str`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::NaiveDate;"] # [doc = ""] # [doc = " assert_eq!(format!(\"{}\", NaiveDate::from_ymd_opt(2015, 9, 5).unwrap()), \"2015-09-05\");"] # [doc = " assert_eq!(format!(\"{}\", NaiveDate::from_ymd_opt(0, 1, 1).unwrap()), \"0000-01-01\");"] # [doc = " assert_eq!(format!(\"{}\", NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()), \"9999-12-31\");"] # [doc = " ```"] # [doc = ""] # [doc = " ISO 8601 requires an explicit sign for years before 1 BCE or after 9999 CE."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::NaiveDate;"] # [doc = " assert_eq!(format!(\"{}\", NaiveDate::from_ymd_opt(-1, 1, 1).unwrap()), \"-0001-01-01\");"] # [doc = " assert_eq!(format!(\"{}\", NaiveDate::from_ymd_opt(10000, 12, 31).unwrap()), \"+10000-12-31\");"] # [doc = " ```"] impl fmt :: Display for NaiveDate { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (self , f) } }
    };
}

impl_363!();