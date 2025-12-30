// Generated macro for impl_34 (impl)
macro_rules! Depcrate_loggingimpl_34 {
() => {
// Module: crate::logging
// Provides: {"impl_34"}
// Dependencies: {}
impl fmt :: Display for ColorLevel { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let level = self . 0 ; if no_color () { write ! (f , "{level:<5}") } else { let color = match level { Level :: Trace => AnsiColor :: Magenta , Level :: Debug => AnsiColor :: Blue , Level :: Info => AnsiColor :: Green , Level :: Warn => AnsiColor :: Yellow , Level :: Error => AnsiColor :: Red , } ; let style = anstyle :: Style :: new () . fg_color (Some (color . into ())) ; write ! (f , "{style}{level:<5}{style:#}") } } }
};
}
