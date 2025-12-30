// Generated macro for local_time (module)
macro_rules! Depcrate_integrations_timelocal_time {
() => {
// Module: crate::integrations::time
// Provides: {"local_time"}
// Dependencies: {}
mod local_time { use super :: * ; # [doc = " Full format of a [`LocalTime` scalar][1]."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] const FORMAT : & [BorrowedFormatItem < '_ >] = format_description ! ("[hour]:[minute]:[second].[subsecond digits:3]") ; # [doc = " Format of a [`LocalTime` scalar][1] without milliseconds."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] const FORMAT_NO_MILLIS : & [BorrowedFormatItem < '_ >] = format_description ! ("[hour]:[minute]:[second]") ; # [doc = " Format of a [`LocalTime` scalar][1] without seconds."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] const FORMAT_NO_SECS : & [BorrowedFormatItem < '_ >] = format_description ! ("[hour]:[minute]") ; impl Display for LazyFmt < & LocalTime > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . format_into (& mut IoAdapter (f) , if self . 0 . millisecond () == 0 { FORMAT_NO_MILLIS } else { FORMAT } ,) . map_err (| _ | fmt :: Error) . map (drop) } } pub (super) fn to_output (v : & LocalTime) -> impl Display { LazyFmt (v) } pub (super) fn from_input (s : & str) -> Result < LocalTime , Box < str > > { LocalTime :: parse (s , FORMAT_NO_MILLIS) . or_else (| _ | LocalTime :: parse (s , FORMAT_NO_SECS)) . or_else (| _ | LocalTime :: parse (s , FORMAT)) . map_err (| e | format ! ("Invalid `LocalTime`: {e}") . into ()) } }
};
}
