macro_rules! deps {
    () => {
        Local!();
        RuleDay!();
        LocalTimeType!();
        Transition!();
    };
}

macro_rules! AlternateTime {
    () => {
        deps!();
        # [doc = " Transition rule representing alternate local time types"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (super) struct AlternateTime { # [doc = " Local time type for standard time"] pub (super) std : LocalTimeType , # [doc = " Local time type for Daylight Saving Time"] pub (super) dst : LocalTimeType , # [doc = " Start day of Daylight Saving Time"] dst_start : RuleDay , # [doc = " Local start day time of Daylight Saving Time, in seconds"] dst_start_time : i32 , # [doc = " End day of Daylight Saving Time"] dst_end : RuleDay , # [doc = " Local end day time of Daylight Saving Time, in seconds"] dst_end_time : i32 , }
    };
}

AlternateTime!();