// Generated macro for impl_94 (impl)
macro_rules! Depcrate_log_formatimpl_94 {
() => {
// Module: crate::log::format
// Provides: {"impl_94"}
// Dependencies: {}
impl FormatterFormat < 'static > { # [doc = " Parse a string into a choice of [`FormatterFormat`]."] # [doc = ""] # [doc = " Unknown strings return `None`."] pub fn from_string (s : & str , with_location : bool) -> Option < FormatterFormat < 'static > > { match s { "default" => Some (FormatterFormat :: Default { with_location }) , "oneline" => Some (FormatterFormat :: OneLine { with_location }) , _ => None , } } # [doc = " Get a list of valid string names for the various format options."] # [doc = ""] # [doc = " This will *not* include an entry for [`FormatterFormat::Custom`] because"] # [doc = " that requires a format string argument."] pub fn get_options () -> impl Iterator < Item = & 'static str > { ["default" , "oneline"] . iter () . cloned () } }
};
}
