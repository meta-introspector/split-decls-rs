// Generated macro for local_time (module)
macro_rules! Depcrate_integrations_jifflocal_time {
() => {
// Module: crate::integrations::jiff
// Provides: {"local_time"}
// Dependencies: {}
mod local_time { use std :: fmt :: Display ; use super :: LocalTime ; # [doc = " Full format of a [`LocalTime` scalar][1]."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] const FORMAT : & str = "%H:%M:%S%.3f" ; # [doc = " Format of a [`LocalTime` scalar][1] without milliseconds."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] const FORMAT_NO_MILLIS : & str = "%H:%M:%S" ; # [doc = " Format of a [`LocalTime` scalar][1] without seconds."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] const FORMAT_NO_SECS : & str = "%H:%M" ; pub (super) fn to_output (v : & LocalTime) -> impl Display { if v . subsec_nanosecond () == 0 { v . strftime (FORMAT_NO_MILLIS) } else { v . strftime (FORMAT) } } pub (super) fn from_input (s : & str) -> Result < LocalTime , Box < str > > { LocalTime :: strptime (FORMAT_NO_MILLIS , s) . or_else (| _ | LocalTime :: strptime (FORMAT_NO_SECS , s)) . or_else (| _ | LocalTime :: strptime (FORMAT , s)) . map_err (| e | format ! ("Invalid `LocalTime`: {e}") . into ()) } }
};
}
