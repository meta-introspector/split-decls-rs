macro_rules! deps {
    () => {
        NaiveDate!();
        Weekday!();
    };
}

macro_rules! NaiveWeek {
    () => {
        deps!();
        # [doc = " A week represented by a [`NaiveDate`] and a [`Weekday`] which is the first"] # [doc = " day of the week."] # [derive (Clone , Copy , Debug , Eq)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct NaiveWeek { date : NaiveDate , start : Weekday , }
    };
}

NaiveWeek!()