// Generated macro for tests (module)
macro_rules! Depcrate_calendartests {
() => {
// Module: crate::calendar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use ratatui_core :: style :: Color ; use time :: Month ; use super :: * ; # [test] fn event_store () { let a = (Date :: from_calendar_date (2023 , Month :: January , 1) . unwrap () , Style :: default () ,) ; let b = (Date :: from_calendar_date (2023 , Month :: January , 2) . unwrap () , Style :: default () . bg (Color :: Red) . fg (Color :: Blue) ,) ; let mut s = CalendarEventStore :: default () ; s . add (b . 0 , b . 1) ; assert_eq ! (s . get_style (a . 0) , a . 1 , "Date not added to the styler should look up as Style::default()") ; assert_eq ! (s . get_style (b . 0) , b . 1 , "Date added to styler should return the provided style") ; } # [test] fn test_today () { CalendarEventStore :: today (Style :: default ()) ; } # [test] fn render_in_minimal_buffer () { let mut buffer = Buffer :: empty (Rect :: new (0 , 0 , 1 , 1)) ; let calendar = Monthly :: new (Date :: from_calendar_date (1984 , Month :: January , 1) . unwrap () , CalendarEventStore :: default () ,) ; calendar . render (buffer . area , & mut buffer) ; assert_eq ! (buffer , Buffer :: with_lines ([" "])) ; } # [test] fn render_in_zero_size_buffer () { let mut buffer = Buffer :: empty (Rect :: ZERO) ; let calendar = Monthly :: new (Date :: from_calendar_date (1984 , Month :: January , 1) . unwrap () , CalendarEventStore :: default () ,) ; calendar . render (buffer . area , & mut buffer) ; } }
};
}
