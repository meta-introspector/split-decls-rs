// Generated macro for impl_36 (impl)
macro_rules! Depcrate_configimpl_36 {
() => {
// Module: crate::config
// Provides: {"impl_36"}
// Dependencies: {}
impl PanicMessage for DefaultPanicMessage { fn display (& self , pi : & std :: panic :: PanicInfo < '_ > , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let theme = & self . 0 ; writeln ! (f , "{}" , "The application panicked (crashed)." . style (theme . panic_header)) ? ; let payload = pi . payload () . downcast_ref :: < String > () . map (String :: as_str) . or_else (| | pi . payload () . downcast_ref :: < & str > () . cloned ()) . unwrap_or ("<non string panic payload>") ; write ! (f , "Message:  ") ? ; writeln ! (f , "{}" , payload . style (theme . panic_message)) ? ; write ! (f , "Location: ") ? ; write ! (f , "{}" , crate :: fmt :: LocationSection (pi . location () , * theme)) ? ; Ok (()) } }
};
}
