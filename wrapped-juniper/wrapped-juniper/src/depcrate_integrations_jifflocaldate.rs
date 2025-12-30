// Generated macro for LocalDate (type)
macro_rules! Depcrate_integrations_jiffLocalDate {
() => {
// Module: crate::integrations::jiff
// Provides: {"LocalDate"}
// Dependencies: {}
# [doc = " Representation of a civil date in the Gregorian calendar."] # [doc = ""] # [doc = " Corresponds to a triple of year, month and day. Every value is guaranteed to be a valid"] # [doc = " Gregorian calendar date. For example, both `2023-02-29` and `2023-11-31` are invalid and cannot"] # [doc = " be represented."] # [doc = ""] # [doc = " [`LocalDate` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`jiff::civil::Date`][2] for details."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-date"] # [doc = " [2]: https://docs.rs/jiff/*/jiff/civil/struct.Date.html"] # [graphql_scalar] # [graphql (with = local_date , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/local-date" ,)] pub type LocalDate = jiff :: civil :: Date ;
};
}
