// Generated macro for DATE_FORMAT (const)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoDATE_FORMAT {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"DATE_FORMAT"}
// Dependencies: {}
# [doc = " Warning to future editors:"] # [doc = " Changes in the following formats need to be kept in sync"] # [doc = " with the formats of the [\"time\"](super::time) module."] # [doc = " We do not need a distinction between whole second and"] # [doc = " subsecond since %.f will only print the dot if needed."] # [doc = " We always print as many subsecond as his given to us,"] # [doc = " this means the subsecond part can be 3, 6 or 9 digits."] const DATE_FORMAT : & str = "%F" ;
};
}
