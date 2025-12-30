// Generated macro for date_time (module)
macro_rules! Depcrate_integrations_jiffdate_time {
() => {
// Module: crate::integrations::jiff
// Provides: {"date_time"}
// Dependencies: {}
mod date_time { use std :: fmt :: Display ; use super :: DateTime ; # [doc = " Format of a [`DateTime` scalar][1]."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/date-time"] const FORMAT : & str = "%Y-%m-%dT%H:%M:%S%.fZ" ; pub (super) fn to_output (v : & DateTime) -> impl Display { v . strftime (FORMAT) } pub (super) fn from_input (s : & str) -> Result < DateTime , Box < str > > { s . parse () . map_err (| e | format ! ("Invalid `DateTime`: {e}") . into ()) } }
};
}
