macro_rules! deps {
    () => {
        IsoWeek!();
        Datelike!();
        NaiveDate!();
    };
}

macro_rules! impl_479 {
    () => {
        deps!();
        # [doc = " The `Debug` output of the ISO week `w` is the same as"] # [doc = " [`d.format(\"%G-W%V\")`](../format/strftime/index.html)"] # [doc = " where `d` is any `NaiveDate` value in that week."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{Datelike, NaiveDate};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     format!(\"{:?}\", NaiveDate::from_ymd_opt(2015, 9, 5).unwrap().iso_week()),"] # [doc = "     \"2015-W36\""] # [doc = " );"] # [doc = " assert_eq!(format!(\"{:?}\", NaiveDate::from_ymd_opt(0, 1, 3).unwrap().iso_week()), \"0000-W01\");"] # [doc = " assert_eq!("] # [doc = "     format!(\"{:?}\", NaiveDate::from_ymd_opt(9999, 12, 31).unwrap().iso_week()),"] # [doc = "     \"9999-W52\""] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " ISO 8601 requires an explicit sign for years before 1 BCE or after 9999 CE."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::{NaiveDate, Datelike};"] # [doc = " assert_eq!(format!(\"{:?}\", NaiveDate::from_ymd_opt(0, 1, 2).unwrap().iso_week()), \"-0001-W52\");"] # [doc = " assert_eq!("] # [doc = "     format!(\"{:?}\", NaiveDate::from_ymd_opt(10000, 12, 31).unwrap().iso_week()),"] # [doc = "     \"+10000-W52\""] # [doc = " );"] # [doc = " ```"] impl fmt :: Debug for IsoWeek { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let year = self . year () ; let week = self . week () ; if (0 ..= 9999) . contains (& year) { write ! (f , "{year:04}-W{week:02}") } else { write ! (f , "{year:+05}-W{week:02}") } } }
    };
}

impl_479!()