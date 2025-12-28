macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! Days {
    () => {
        deps!();
        # [doc = " A duration in calendar days."] # [doc = ""] # [doc = " This is useful because when using `TimeDelta` it is possible that adding `TimeDelta::days(1)`"] # [doc = " doesn't increment the day value as expected due to it being a fixed number of seconds. This"] # [doc = " difference applies only when dealing with `DateTime<TimeZone>` data types and in other cases"] # [doc = " `TimeDelta::days(n)` and `Days::new(n)` are equivalent."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct Days (pub (crate) u64) ;
    };
}

Days!()