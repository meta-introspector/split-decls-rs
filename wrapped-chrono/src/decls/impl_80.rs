macro_rules! deps {
    () => {
        NaiveTime!();
        FixedOffset!();
        DstTester!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl DstTester { fn winter_offset () -> FixedOffset { FixedOffset :: east_opt (8 * 60 * 60) . unwrap () } fn summer_offset () -> FixedOffset { FixedOffset :: east_opt (9 * 60 * 60) . unwrap () } const TO_WINTER_MONTH_DAY : (u32 , u32) = (4 , 15) ; const TO_SUMMER_MONTH_DAY : (u32 , u32) = (9 , 15) ; fn transition_start_local () -> NaiveTime { NaiveTime :: from_hms_opt (2 , 0 , 0) . unwrap () } }
    };
}

impl_80!()