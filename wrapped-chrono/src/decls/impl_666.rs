macro_rules! deps {
    () => {
        TimeZone!();
        Utc!();
        Offset!();
        NaiveDate!();
        NaiveDateTime!();
        MappedLocalTime!();
    };
}

macro_rules! impl_666 {
    () => {
        deps!();
        impl TimeZone for Utc { type Offset = Utc ; fn from_offset (_state : & Utc) -> Utc { Utc } fn offset_from_local_date (& self , _local : & NaiveDate) -> MappedLocalTime < Utc > { MappedLocalTime :: Single (Utc) } fn offset_from_local_datetime (& self , _local : & NaiveDateTime) -> MappedLocalTime < Utc > { MappedLocalTime :: Single (Utc) } fn offset_from_utc_date (& self , _utc : & NaiveDate) -> Utc { Utc } fn offset_from_utc_datetime (& self , _utc : & NaiveDateTime) -> Utc { Utc } }
    };
}

impl_666!()