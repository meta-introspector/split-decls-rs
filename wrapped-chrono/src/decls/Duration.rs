macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! Duration {
    () => {
        deps!();
        # [doc = " Alias of [`TimeDelta`]."] pub type Duration = TimeDelta ;
    };
}

Duration!()