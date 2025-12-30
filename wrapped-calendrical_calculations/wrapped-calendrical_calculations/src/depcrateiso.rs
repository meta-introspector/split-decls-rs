// Generated macro for iso (module)
macro_rules! Depcrateiso {
() => {
// Module: crate
// Provides: {"iso"}
// Dependencies: {}
# [doc = " Aliases for the Gregorian calendar."] # [doc = ""] # [doc = " This is *not* the ISO week calendar as described in the book."] # [deprecated (since = "0.2.3" , note = "use `gregorian`")] pub mod iso { pub use crate :: gregorian :: day_before_year ; pub use crate :: gregorian :: days_before_month ; pub use crate :: gregorian :: easter ; pub use crate :: gregorian :: fixed_from_gregorian as const_fixed_from_iso ; pub use crate :: gregorian :: fixed_from_gregorian as fixed_from_iso ; pub use crate :: gregorian :: gregorian_from_fixed as iso_from_fixed ; pub use crate :: gregorian :: is_leap_year ; pub use crate :: gregorian :: year_day ; pub use crate :: gregorian :: year_from_fixed as iso_year_from_fixed ; }
};
}
