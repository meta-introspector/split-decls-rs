macro_rules! deps {
    () => {
        NaiveTime!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        # [doc = " The default value for a NaiveTime is midnight, 00:00:00 exactly."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use chrono::NaiveTime;"] # [doc = ""] # [doc = " let default_time = NaiveTime::default();"] # [doc = " assert_eq!(default_time, NaiveTime::from_hms_opt(0, 0, 0).unwrap());"] # [doc = " ```"] impl Default for NaiveTime { fn default () -> Self { NaiveTime :: from_hms_opt (0 , 0 , 0) . unwrap () } }
    };
}

impl_523!()