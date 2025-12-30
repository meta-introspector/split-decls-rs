// Generated macro for next_month (function)
macro_rules! Depcratenext_month {
() => {
// Module: crate
// Provides: {"next_month"}
// Dependencies: {}
fn next_month (date : Date) -> Date { if date . month () == Month :: December { date . replace_month (Month :: January) . unwrap () . replace_year (date . year () + 1) . unwrap () } else { date . replace_month (date . month () . next ()) . unwrap () } }
};
}
