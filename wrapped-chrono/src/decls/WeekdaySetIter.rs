macro_rules! deps {
    () => {
        WeekdaySet!();
        Weekday!();
    };
}

macro_rules! WeekdaySetIter {
    () => {
        deps!();
        # [doc = " An iterator over a collection of weekdays, starting from a given day."] # [doc = ""] # [doc = " See [`WeekdaySet::iter()`]."] # [derive (Debug , Clone)] pub struct WeekdaySetIter { days : WeekdaySet , start : Weekday , }
    };
}

WeekdaySetIter!();