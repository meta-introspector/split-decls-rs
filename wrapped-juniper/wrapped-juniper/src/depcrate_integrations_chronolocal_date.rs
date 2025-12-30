// Generated macro for local_date (module)
macro_rules! Depcrate_integrations_chronolocal_date {
() => {
// Module: crate::integrations::chrono
// Provides: {"local_date"}
// Dependencies: {}
mod local_date { use std :: fmt :: Display ; use super :: LocalDate ; # [doc = " Format of a [`LocalDate` scalar][1]."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-date"] const FORMAT : & str = "%Y-%m-%d" ; pub (super) fn to_output (v : & LocalDate) -> impl Display { v . format (FORMAT) } pub (super) fn from_input (s : & str) -> Result < LocalDate , Box < str > > { LocalDate :: parse_from_str (s , FORMAT) . map_err (| e | format ! ("Invalid `LocalDate`: {e}") . into ()) } }
};
}
