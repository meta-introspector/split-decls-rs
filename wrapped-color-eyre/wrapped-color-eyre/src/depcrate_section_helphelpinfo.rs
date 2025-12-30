// Generated macro for HelpInfo (enum)
macro_rules! Depcrate_section_helpHelpInfo {
() => {
// Module: crate::section::help
// Provides: {"HelpInfo"}
// Dependencies: {}
pub (crate) enum HelpInfo { Error (Box < dyn std :: error :: Error + Send + Sync + 'static > , Theme) , Custom (Box < dyn Display + Send + Sync + 'static >) , Note (Box < dyn Display + Send + Sync + 'static > , Theme) , Warning (Box < dyn Display + Send + Sync + 'static > , Theme) , Suggestion (Box < dyn Display + Send + Sync + 'static > , Theme) , }
};
}
