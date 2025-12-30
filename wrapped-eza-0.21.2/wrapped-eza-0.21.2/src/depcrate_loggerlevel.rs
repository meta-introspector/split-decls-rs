// Generated macro for level (function)
macro_rules! Depcrate_loggerlevel {
() => {
// Module: crate::logger
// Provides: {"level"}
// Dependencies: {}
fn level (level : log :: Level) -> ANSIString < 'static > { # [rustfmt :: skip] return match level { log :: Level :: Error => Colour :: Red . paint ("ERROR") , log :: Level :: Warn => Colour :: Yellow . paint ("WARN") , log :: Level :: Info => Colour :: Cyan . paint ("INFO") , log :: Level :: Debug => Colour :: Blue . paint ("DEBUG") , log :: Level :: Trace => Colour :: Fixed (245) . paint ("TRACE") , } ; }
};
}
