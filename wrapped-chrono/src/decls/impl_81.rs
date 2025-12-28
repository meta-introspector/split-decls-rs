macro_rules! deps {
    () => {
        TimeZone!();
        NaiveDateTime!();
        TimeDelta!();
        DstTester!();
        FixedOffset!();
        MappedLocalTime!();
        Offset!();
        NaiveDate!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl TimeZone for DstTester { type Offset = FixedOffset ; fn from_offset (_ : & Self :: Offset) -> Self { DstTester } fn offset_from_local_date (& self , _ : & NaiveDate) -> crate :: MappedLocalTime < Self :: Offset > { unimplemented ! () } fn offset_from_local_datetime (& self , local : & NaiveDateTime ,) -> crate :: MappedLocalTime < Self :: Offset > { let local_to_winter_transition_start = NaiveDate :: from_ymd_opt (local . year () , DstTester :: TO_WINTER_MONTH_DAY . 0 , DstTester :: TO_WINTER_MONTH_DAY . 1 ,) . unwrap () . and_time (DstTester :: transition_start_local ()) ; let local_to_winter_transition_end = NaiveDate :: from_ymd_opt (local . year () , DstTester :: TO_WINTER_MONTH_DAY . 0 , DstTester :: TO_WINTER_MONTH_DAY . 1 ,) . unwrap () . and_time (DstTester :: transition_start_local () - TimeDelta :: try_hours (1) . unwrap ()) ; let local_to_summer_transition_start = NaiveDate :: from_ymd_opt (local . year () , DstTester :: TO_SUMMER_MONTH_DAY . 0 , DstTester :: TO_SUMMER_MONTH_DAY . 1 ,) . unwrap () . and_time (DstTester :: transition_start_local ()) ; let local_to_summer_transition_end = NaiveDate :: from_ymd_opt (local . year () , DstTester :: TO_SUMMER_MONTH_DAY . 0 , DstTester :: TO_SUMMER_MONTH_DAY . 1 ,) . unwrap () . and_time (DstTester :: transition_start_local () + TimeDelta :: try_hours (1) . unwrap ()) ; if * local < local_to_winter_transition_end || * local >= local_to_summer_transition_end { MappedLocalTime :: Single (DstTester :: summer_offset ()) } else if * local >= local_to_winter_transition_start && * local < local_to_summer_transition_start { MappedLocalTime :: Single (DstTester :: winter_offset ()) } else if * local >= local_to_winter_transition_end && * local < local_to_winter_transition_start { MappedLocalTime :: Ambiguous (DstTester :: winter_offset () , DstTester :: summer_offset ()) } else if * local >= local_to_summer_transition_start && * local < local_to_summer_transition_end { MappedLocalTime :: None } else { panic ! ("Unexpected local time {local}") } } fn offset_from_utc_date (& self , _ : & NaiveDate) -> Self :: Offset { unimplemented ! () } fn offset_from_utc_datetime (& self , utc : & NaiveDateTime) -> Self :: Offset { let utc_to_winter_transition = NaiveDate :: from_ymd_opt (utc . year () , DstTester :: TO_WINTER_MONTH_DAY . 0 , DstTester :: TO_WINTER_MONTH_DAY . 1 ,) . unwrap () . and_time (DstTester :: transition_start_local ()) - DstTester :: summer_offset () ; let utc_to_summer_transition = NaiveDate :: from_ymd_opt (utc . year () , DstTester :: TO_SUMMER_MONTH_DAY . 0 , DstTester :: TO_SUMMER_MONTH_DAY . 1 ,) . unwrap () . and_time (DstTester :: transition_start_local ()) - DstTester :: winter_offset () ; if * utc < utc_to_winter_transition || * utc >= utc_to_summer_transition { DstTester :: summer_offset () } else if * utc >= utc_to_winter_transition && * utc < utc_to_summer_transition { DstTester :: winter_offset () } else { panic ! ("Unexpected utc time {utc}") } } }
    };
}

impl_81!();