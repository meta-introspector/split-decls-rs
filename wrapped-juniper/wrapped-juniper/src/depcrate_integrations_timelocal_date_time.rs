// Generated macro for local_date_time (module)
macro_rules! Depcrate_integrations_timelocal_date_time {
() => {
// Module: crate::integrations::time
// Provides: {"local_date_time"}
// Dependencies: {}
mod local_date_time { use super :: * ; # [doc = " Format of a [`LocalDateTime` scalar][1]."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-date-time"] const FORMAT : & [BorrowedFormatItem < '_ >] = format_description ! ("[year]-[month]-[day]T[hour]:[minute]:[second]") ; impl Display for LazyFmt < & LocalDateTime > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . format_into (& mut IoAdapter (f) , FORMAT) . map_err (| _ | fmt :: Error) . map (drop) } } pub (super) fn to_output (v : & LocalDateTime) -> impl Display { LazyFmt (v) } pub (super) fn from_input (s : & str) -> Result < LocalDateTime , Box < str > > { LocalDateTime :: parse (s , FORMAT) . map_err (| e | format ! ("Invalid `LocalDateTime`: {e}") . into ()) } }
};
}
