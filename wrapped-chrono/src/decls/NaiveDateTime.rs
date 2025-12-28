macro_rules! deps {
    () => {
        Timelike!();
        Weekday!();
        NaiveDate!();
        NaiveTime!();
    };
}

macro_rules! NaiveDateTime {
    () => {
        deps!();
        # [doc = " ISO 8601 combined date and time without timezone."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " `NaiveDateTime` is commonly created from [`NaiveDate`]."] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{NaiveDate, NaiveDateTime};"] # [doc = ""] # [doc = " let dt: NaiveDateTime ="] # [doc = "     NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();"] # [doc = " # let _ = dt;"] # [doc = " ```"] # [doc = ""] # [doc = " You can use typical [date-like](Datelike) and [time-like](Timelike) methods,"] # [doc = " provided that relevant traits are in the scope."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::{NaiveDate, NaiveDateTime};"] # [doc = " # let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap();"] # [doc = " use chrono::{Datelike, Timelike, Weekday};"] # [doc = ""] # [doc = " assert_eq!(dt.weekday(), Weekday::Fri);"] # [doc = " assert_eq!(dt.num_seconds_from_midnight(), 33011);"] # [doc = " ```"] # [derive (PartialEq , Eq , Hash , PartialOrd , Ord , Copy , Clone)] # [cfg_attr (any (feature = "rkyv" , feature = "rkyv-16" , feature = "rkyv-32" , feature = "rkyv-64") , derive (Archive , Deserialize , Serialize) , archive (compare (PartialEq , PartialOrd)) , archive_attr (derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Debug , Hash)))] # [cfg_attr (feature = "rkyv-validation" , archive (check_bytes))] # [cfg_attr (all (feature = "arbitrary" , feature = "std") , derive (arbitrary :: Arbitrary))] pub struct NaiveDateTime { date : NaiveDate , time : NaiveTime , }
    };
}

NaiveDateTime!();