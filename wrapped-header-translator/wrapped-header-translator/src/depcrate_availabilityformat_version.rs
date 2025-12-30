// Generated macro for format_version (function)
macro_rules! Depcrate_availabilityformat_version {
() => {
// Module: crate::availability
// Provides: {"format_version"}
// Dependencies: {}
fn format_version (version : Version) -> impl Display { FormatterFn (move | f | { write ! (f , "{}" , version . x) ? ; if let Some (y) = version . y { write ! (f , ".{}" , y) ? ; if let Some (z) = version . z { write ! (f , ".{}" , z) ? ; } } else if let Some (z) = version . z { write ! (f , ".0.{}" , z) ? ; } Ok (()) }) }
};
}
