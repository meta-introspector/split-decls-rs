// Generated macro for impl_105 (impl)
macro_rules! Depcrate_section_helpimpl_105 {
() => {
// Module: crate::section::help
// Provides: {"impl_105"}
// Dependencies: {}
impl fmt :: Debug for HelpInfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { HelpInfo :: Note (note , ..) => f . debug_tuple ("Note") . field (& format_args ! ("{note}")) . finish () , HelpInfo :: Warning (warning , ..) => f . debug_tuple ("Warning") . field (& format_args ! ("{warning}")) . finish () , HelpInfo :: Suggestion (suggestion , ..) => f . debug_tuple ("Suggestion") . field (& format_args ! ("{suggestion}")) . finish () , HelpInfo :: Custom (custom , ..) => f . debug_tuple ("CustomSection") . field (& format_args ! ("{custom}")) . finish () , HelpInfo :: Error (error , ..) => f . debug_tuple ("Error") . field (error) . finish () , } } }
};
}
