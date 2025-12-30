// Generated macro for utc_offset (module)
macro_rules! Depcrate_integrations_timeutc_offset {
() => {
// Module: crate::integrations::time
// Provides: {"utc_offset"}
// Dependencies: {}
mod utc_offset { use super :: * ; impl Display for LazyFmt < & UtcOffset > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . format_into (& mut IoAdapter (f) , UTC_OFFSET_FORMAT) . map_err (| _ | fmt :: Error) . map (drop) } } pub (super) fn to_output (v : & UtcOffset) -> impl Display { LazyFmt (v) } pub (super) fn from_input (s : & str) -> Result < UtcOffset , Box < str > > { UtcOffset :: parse (s , UTC_OFFSET_FORMAT) . map_err (| e | format ! ("Invalid `UtcOffset`: {e}") . into ()) } }
};
}
