macro_rules! deps {
    () => {
        NaiveDate!();
        TimeZone!();
        Offset!();
        NaiveDateTime!();
        Local!();
        MappedLocalTime!();
        FixedOffset!();
        NaiveTime!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl TimeZone for Local { type Offset = FixedOffset ; fn from_offset (_offset : & FixedOffset) -> Local { Local } # [allow (deprecated)] fn offset_from_local_date (& self , local : & NaiveDate) -> MappedLocalTime < FixedOffset > { self . offset_from_local_datetime (& local . and_time (NaiveTime :: MIN)) } fn offset_from_local_datetime (& self , local : & NaiveDateTime) -> MappedLocalTime < FixedOffset > { inner :: offset_from_local_datetime (local) } # [allow (deprecated)] fn offset_from_utc_date (& self , utc : & NaiveDate) -> FixedOffset { self . offset_from_utc_datetime (& utc . and_time (NaiveTime :: MIN)) } fn offset_from_utc_datetime (& self , utc : & NaiveDateTime) -> FixedOffset { inner :: offset_from_utc_datetime (utc) . unwrap () } }
    };
}

impl_656!();