// Generated macro for Style (trait)
macro_rules! Depcrate_log_formatStyle {
() => {
// Module: crate::log::format
// Provides: {"Style"}
// Dependencies: {}
# [doc = " Describes one of the fixed format string sets."] trait Style { const FORMAT : & 'static str ; const FORMAT_WITH_TS : & 'static str ; const FORMAT_WITH_LOC : & 'static str ; const FORMAT_WITH_TS_LOC : & 'static str ; # [doc = " Return a suitable format string, given these options."] fn get_string (with_location : bool , has_timestamp : bool) -> & 'static str { match (with_location , has_timestamp) { (false , false) => Self :: FORMAT , (false , true) => Self :: FORMAT_WITH_TS , (true , false) => Self :: FORMAT_WITH_LOC , (true , true) => Self :: FORMAT_WITH_TS_LOC , } } }
};
}
