macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        # [doc = " The `Debug` output of the naive date `d` is the same as"] # [doc = " [`d.format(\"%Y-%m-%d\")`](crate::format::strftime)."] # [doc = ""] # [doc = " The string printed can be readily parsed via the `parse` method on `str`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::NaiveDate;"] # [doc = ""] # [doc = " assert_eq!(format!(\"{:?}\", NaiveDate::from_ymd_opt(2015, 9, 5).unwrap()), \"2015-09-05\");"] # [doc = " assert_eq!(format!(\"{:?}\", NaiveDate::from_ymd_opt(0, 1, 1).unwrap()), \"0000-01-01\");"] # [doc = " assert_eq!(format!(\"{:?}\", NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()), \"9999-12-31\");"] # [doc = " ```"] # [doc = ""] # [doc = " ISO 8601 requires an explicit sign for years before 1 BCE or after 9999 CE."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::NaiveDate;"] # [doc = " assert_eq!(format!(\"{:?}\", NaiveDate::from_ymd_opt(-1, 1, 1).unwrap()), \"-0001-01-01\");"] # [doc = " assert_eq!(format!(\"{:?}\", NaiveDate::from_ymd_opt(10000, 12, 31).unwrap()), \"+10000-12-31\");"] # [doc = " ```"] impl fmt :: Debug for NaiveDate { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use core :: fmt :: Write ; let year = self . year () ; let mdf = self . mdf () ; if (0 ..= 9999) . contains (& year) { write_hundreds (f , (year / 100) as u8) ? ; write_hundreds (f , (year % 100) as u8) ? ; } else { write ! (f , "{year:+05}") ? ; } f . write_char ('-') ? ; write_hundreds (f , mdf . month () as u8) ? ; f . write_char ('-') ? ; write_hundreds (f , mdf . day () as u8) } }
    };
}

impl_361!()