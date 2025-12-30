// Generated macro for local_date (module)
macro_rules! Depcrate_integrations_timelocal_date {
() => {
// Module: crate::integrations::time
// Provides: {"local_date"}
// Dependencies: {}
mod local_date { use super :: * ; # [doc = " Format of a [`LocalDate` scalar][1]."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-date"] const FORMAT : & [BorrowedFormatItem < '_ >] = format_description ! ("[year]-[month]-[day]") ; impl Display for LazyFmt < & LocalDate > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . format_into (& mut IoAdapter (f) , FORMAT) . map_err (| _ | fmt :: Error) . map (drop) } } pub (super) fn to_output (v : & LocalDate) -> impl Display { LazyFmt (v) } pub (super) fn from_input (s : & str) -> Result < LocalDate , Box < str > > { LocalDate :: parse (s , FORMAT) . map_err (| e | format ! ("Invalid `LocalDate`: {e}") . into ()) } }
};
}
