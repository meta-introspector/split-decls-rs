macro_rules! deps {
    () => {
        NaiveDateTime!();
        DurationRound!();
        TimeDelta!();
        RoundingError!();
    };
}

macro_rules! impl_686 {
    () => {
        deps!();
        impl DurationRound for NaiveDateTime { type Err = RoundingError ; fn duration_round (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_round (self , self , duration) } fn duration_trunc (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_trunc (self , self , duration) } fn duration_round_up (self , duration : TimeDelta) -> Result < Self , Self :: Err > { duration_round_up (self , self , duration) } }
    };
}

impl_686!();