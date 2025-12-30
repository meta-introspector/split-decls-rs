// Generated macro for impl_87 (impl)
macro_rules! Depcrate_log_formatimpl_87 {
() => {
// Module: crate::log::format
// Provides: {"impl_87"}
// Dependencies: {}
impl Formatter { # [doc = " Create a new formatter, using the given configuration."] pub fn new (config : FormatterConfig) -> Self { Self { formatter : InternalFormatter :: new (config , Source :: Defmt) , } } # [doc = " Format a defmt frame using this formatter."] pub fn format_frame < 'a > (& self , frame : Frame < 'a > , file : Option < & 'a str > , line : Option < u32 > , module_path : Option < & str > ,) -> String { let (timestamp , level) = super :: timestamp_and_level_from_frame (& frame) ; # [allow (clippy :: match_single_binding)] match format_args ! ("{}" , frame . display_message ()) { args => { let log_record = & LogRecord :: builder () . args (args) . module_path (module_path) . file (file) . line (line) . build () ; let record = DefmtRecord { log_record , payload : Payload { level , timestamp } , } ; self . format (& record) } } } # [doc = " Format the given [`DefmtRecord`] (which is an internal type)."] pub (super) fn format (& self , record : & DefmtRecord) -> String { self . formatter . format (& Record :: Defmt (record)) } }
};
}
