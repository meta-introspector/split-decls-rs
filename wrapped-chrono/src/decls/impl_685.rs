macro_rules! deps {
    () => {
        TimeDelta!();
        DateTime!();
        TimeZone!();
        RoundingError!();
        DurationRound!();
    };
}

macro_rules! impl_685 {
    () => {
        deps!();
        impl < Tz : TimeZone > DurationRound for DateTime < Tz > { type Err = RoundingError ; fn duration_round (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_round (self . naive_local () , self , duration) } fn duration_trunc (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_trunc (self . naive_local () , self , duration) } fn duration_round_up (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_round_up (self . naive_local () , self , duration) } }
    };
}

impl_685!()