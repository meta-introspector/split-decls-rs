// Generated macro for events (function)
macro_rules! Depcrateevents {
() => {
// Module: crate
// Provides: {"events"}
// Dependencies: {}
# [doc = " Makes a list of dates for the current year."] fn events (selected_date : Date) -> Result < CalendarEventStore > { const SELECTED : Style = Style :: new () . fg (Color :: White) . bg (Color :: Red) . add_modifier (Modifier :: BOLD) ; const HOLIDAY : Style = Style :: new () . fg (Color :: Red) . add_modifier (Modifier :: UNDERLINED) ; const SEASON : Style = Style :: new () . fg (Color :: Green) . bg (Color :: Black) . add_modifier (Modifier :: UNDERLINED) ; let mut list = CalendarEventStore :: today (Style :: default () . add_modifier (Modifier :: BOLD) . bg (Color :: Blue) ,) ; let y = selected_date . year () ; list . add (Date :: from_calendar_date (y , Month :: January , 1) ? , HOLIDAY) ; list . add (Date :: from_calendar_date (y + 1 , Month :: January , 1) ? , HOLIDAY) ; list . add (Date :: from_calendar_date (y , Month :: February , 2) ? , HOLIDAY) ; list . add (Date :: from_calendar_date (y , Month :: April , 1) ? , HOLIDAY) ; list . add (Date :: from_calendar_date (y , Month :: April , 22) ? , HOLIDAY) ; list . add (Date :: from_calendar_date (y , Month :: May , 4) ? , HOLIDAY) ; list . add (Date :: from_calendar_date (y , Month :: December , 23) ? , HOLIDAY) ; list . add (Date :: from_calendar_date (y , Month :: December , 31) ? , HOLIDAY) ; list . add (Date :: from_calendar_date (y , Month :: March , 22) ? , SEASON) ; list . add (Date :: from_calendar_date (y , Month :: June , 21) ? , SEASON) ; list . add (Date :: from_calendar_date (y , Month :: September , 22) ? , SEASON) ; list . add (Date :: from_calendar_date (y , Month :: December , 21) ? , SEASON) ; list . add (selected_date , SELECTED) ; Ok (list) }
};
}
