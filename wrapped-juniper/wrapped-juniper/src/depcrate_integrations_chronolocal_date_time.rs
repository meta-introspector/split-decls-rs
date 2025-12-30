// Generated macro for local_date_time (module)
macro_rules! Depcrate_integrations_chronolocal_date_time {
() => {
// Module: crate::integrations::chrono
// Provides: {"local_date_time"}
// Dependencies: {}
mod local_date_time { use std :: fmt :: Display ; use super :: LocalDateTime ; # [doc = " Format of a [`LocalDateTime` scalar][1]."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-date-time"] const FORMAT : & str = "%Y-%m-%dT%H:%M:%S" ; pub (super) fn to_output (v : & LocalDateTime) -> impl Display { v . format (FORMAT) } pub (super) fn from_input (s : & str) -> Result < LocalDateTime , Box < str > > { LocalDateTime :: parse_from_str (s , FORMAT) . map_err (| e | format ! ("Invalid `LocalDateTime`: {e}") . into ()) } }
};
}
