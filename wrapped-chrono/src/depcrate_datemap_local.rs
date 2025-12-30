// Generated macro for map_local (function)
macro_rules! Depcrate_datemap_local {
() => {
// Module: crate::date
// Provides: {"map_local"}
// Dependencies: {}
# [doc = " Maps the local date to other date with given conversion function."] fn map_local < Tz : TimeZone , F > (d : & Date < Tz > , mut f : F) -> Option < Date < Tz > > where F : FnMut (NaiveDate) -> Option < NaiveDate > , { f (d . naive_local ()) . and_then (| date | d . timezone () . from_local_date (& date) . single ()) }
};
}
