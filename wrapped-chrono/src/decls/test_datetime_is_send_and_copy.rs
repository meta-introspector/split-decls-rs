macro_rules! deps {
    () => {
        FixedOffset!();
        DateTime!();
        Offset!();
        NaiveDate!();
        TimeZone!();
        NaiveDateTime!();
        MappedLocalTime!();
    };
}

macro_rules! test_datetime_is_send_and_copy {
    () => {
        deps!();
        # [test] fn test_datetime_is_send_and_copy () { # [derive (Clone)] struct Tz { _not_send : * const i32 , } impl TimeZone for Tz { type Offset = Off ; fn from_offset (_ : & Self :: Offset) -> Self { unimplemented ! () } fn offset_from_local_date (& self , _ : & NaiveDate) -> crate :: MappedLocalTime < Self :: Offset > { unimplemented ! () } fn offset_from_local_datetime (& self , _ : & NaiveDateTime ,) -> crate :: MappedLocalTime < Self :: Offset > { unimplemented ! () } fn offset_from_utc_date (& self , _ : & NaiveDate) -> Self :: Offset { unimplemented ! () } fn offset_from_utc_datetime (& self , _ : & NaiveDateTime) -> Self :: Offset { unimplemented ! () } } # [derive (Copy , Clone , Debug)] struct Off (()) ; impl Offset for Off { fn fix (& self) -> FixedOffset { unimplemented ! () } } fn _assert_send_copy < T : Send + Copy > () { } _assert_send_copy :: < DateTime < Tz > > () ; }
    };
}

test_datetime_is_send_and_copy!();