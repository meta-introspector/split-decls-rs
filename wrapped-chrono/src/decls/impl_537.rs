macro_rules! deps {
    () => {
        MappedLocalTime!();
        NaiveDateTime!();
        TimeZone!();
        FixedOffset!();
        Offset!();
        NaiveDate!();
    };
}

macro_rules! impl_537 {
    () => {
        deps!();
        impl TimeZone for FixedOffset { type Offset = FixedOffset ; fn from_offset (offset : & FixedOffset) -> FixedOffset { * offset } fn offset_from_local_date (& self , _local : & NaiveDate) -> MappedLocalTime < FixedOffset > { MappedLocalTime :: Single (* self) } fn offset_from_local_datetime (& self , _local : & NaiveDateTime) -> MappedLocalTime < FixedOffset > { MappedLocalTime :: Single (* self) } fn offset_from_utc_date (& self , _utc : & NaiveDate) -> FixedOffset { * self } fn offset_from_utc_datetime (& self , _utc : & NaiveDateTime) -> FixedOffset { * self } }
    };
}

impl_537!();