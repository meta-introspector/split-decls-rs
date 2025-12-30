// Generated macro for date_time (module)
macro_rules! Depcrate_integrations_timedate_time {
() => {
// Module: crate::integrations::time
// Provides: {"date_time"}
// Dependencies: {}
mod date_time { use super :: * ; impl Display for LazyFmt < & DateTime > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . to_offset (UtcOffset :: UTC) . format_into (& mut IoAdapter (f) , & Rfc3339) . map_err (| _ | fmt :: Error) . map (drop) } } pub (super) fn to_output (v : & DateTime) -> impl Display { LazyFmt (v) } pub (super) fn from_input (s : & str) -> Result < DateTime , Box < str > > { DateTime :: parse (s , & Rfc3339) . map (| dt | dt . to_offset (UtcOffset :: UTC)) . map_err (| e | format ! ("Invalid `DateTime`: {e}") . into ()) } }
};
}
