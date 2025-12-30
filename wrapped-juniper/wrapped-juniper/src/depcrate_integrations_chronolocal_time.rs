// Generated macro for local_time (module)
macro_rules! Depcrate_integrations_chronolocal_time {
() => {
// Module: crate::integrations::chrono
// Provides: {"local_time"}
// Dependencies: {}
mod local_time { use std :: fmt :: Display ; use chrono :: Timelike as _ ; use super :: LocalTime ; # [doc = " Full format of a [`LocalTime` scalar][1]."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] const FORMAT : & str = "%H:%M:%S%.3f" ; # [doc = " Format of a [`LocalTime` scalar][1] without milliseconds."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] const FORMAT_NO_MILLIS : & str = "%H:%M:%S" ; # [doc = " Format of a [`LocalTime` scalar][1] without seconds."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] const FORMAT_NO_SECS : & str = "%H:%M" ; pub (super) fn to_output (v : & LocalTime) -> impl Display { if v . nanosecond () == 0 { v . format (FORMAT_NO_MILLIS) } else { v . format (FORMAT) } } pub (super) fn from_input (s : & str) -> Result < LocalTime , Box < str > > { LocalTime :: parse_from_str (s , FORMAT_NO_MILLIS) . or_else (| _ | LocalTime :: parse_from_str (s , FORMAT_NO_SECS)) . or_else (| _ | LocalTime :: parse_from_str (s , FORMAT)) . map_err (| e | format ! ("Invalid `LocalTime`: {e}") . into ()) } }
};
}
