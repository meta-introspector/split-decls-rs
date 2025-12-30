// Generated macro for impl_215 (impl)
macro_rules! Depcrate_loggerimpl_215 {
() => {
// Module: crate::logger
// Provides: {"impl_215"}
// Dependencies: {}
impl log :: Log for Logger { fn enabled (& self , _ : & log :: Metadata < '_ >) -> bool { true } fn log (& self , record : & log :: Record < '_ >) { let open = Colour :: Fixed (243) . paint ("[") ; let level = level (record . level ()) ; let close = Colour :: Fixed (243) . paint ("]") ; eprintln ! ("{}{} {}{} {}" , open , level , record . target () , close , record . args ()) ; } fn flush (& self) { } }
};
}
