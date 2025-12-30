// Generated macro for print_events (function)
macro_rules! Depcrateprint_events {
() => {
// Module: crate
// Provides: {"print_events"}
// Dependencies: {}
# [doc = " Print Markdown events with indentation."] # [doc = ""] # [doc = " The `text` label indicates the source of the events."] pub fn print_events (text : & str , events : & [Event]) { eprintln ! ("{text:?} -> [") ; let mut width = 0 ; for event in events { if let Event :: End (_) = event { width -= 2 ; } eprintln ! ("  {:width$}{event:?}" , "") ; if let Event :: Start (_) = event { width += 2 ; } } eprintln ! ("]") ; }
};
}
