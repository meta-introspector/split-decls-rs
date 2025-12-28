macro_rules! deps {
    () => {
        NaiveWeek!();
    };
}

macro_rules! impl_527 {
    () => {
        deps!();
        impl PartialEq for NaiveWeek { fn eq (& self , other : & Self) -> bool { self . first_day () == other . first_day () } }
    };
}

impl_527!();