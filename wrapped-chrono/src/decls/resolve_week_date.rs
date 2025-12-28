macro_rules! deps {
    () => {
        Weekday!();
        NaiveDate!();
        ParseResult!();
    };
}

macro_rules! resolve_week_date {
    () => {
        deps!();
        # [doc = " Create a `NaiveDate` when given a year, week, weekday, and the definition at which day of the"] # [doc = " week a week starts."] # [doc = ""] # [doc = " Returns `IMPOSSIBLE` if `week` is `0` or `53` and the `weekday` falls outside the year."] fn resolve_week_date (year : i32 , week : u32 , weekday : Weekday , week_start_day : Weekday ,) -> ParseResult < NaiveDate > { if week > 53 { return Err (OUT_OF_RANGE) ; } let first_day_of_year = NaiveDate :: from_yo_opt (year , 1) . ok_or (OUT_OF_RANGE) ? ; let first_week_start = 1 + week_start_day . days_since (first_day_of_year . weekday ()) as i32 ; let weekday = weekday . days_since (week_start_day) as i32 ; let ordinal = first_week_start + (week as i32 - 1) * 7 + weekday ; if ordinal <= 0 { return Err (IMPOSSIBLE) ; } first_day_of_year . with_ordinal (ordinal as u32) . ok_or (IMPOSSIBLE) }
    };
}

resolve_week_date!();