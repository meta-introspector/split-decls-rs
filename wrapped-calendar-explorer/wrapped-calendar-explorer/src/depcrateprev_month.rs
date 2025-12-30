// Generated macro for prev_month (function)
macro_rules! Depcrateprev_month {
() => {
// Module: crate
// Provides: {"prev_month"}
// Dependencies: {}
fn prev_month (date : Date) -> Date { if date . month () == Month :: January { date . replace_month (Month :: December) . unwrap () . replace_year (date . year () - 1) . unwrap () } else { date . replace_month (date . month () . previous ()) . unwrap () } }
};
}
