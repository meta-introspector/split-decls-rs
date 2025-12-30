// Generated macro for DATE_FORMAT (const)
macro_rules! Depcrate_sqlite_types_date_and_time_timeDATE_FORMAT {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"DATE_FORMAT"}
// Dependencies: {}
# [doc = " Warning to future editors:"] # [doc = " Changes in the following formats need to be kept in sync"] # [doc = " with the formats of the [\"chrono\"](super::chrono) module."] # [doc = " We need a distinction between whole second and subsecond"] # [doc = " since there is no format option to forgo the dot."] # [doc = " We always print as many subsecond as his given to us,"] # [doc = " this means the subsecond part can be between 1 and 9 digits."] # [doc = ""] # [allow (deprecated)] const DATE_FORMAT : & [FormatItem < '_ >] = format_description ! ("[year]-[month]-[day]") ;
};
}
