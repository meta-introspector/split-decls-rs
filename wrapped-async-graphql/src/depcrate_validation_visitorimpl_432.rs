// Generated macro for impl_432 (impl)
macro_rules! Depcrate_validation_visitorimpl_432 {
() => {
// Module: crate::validation::visitor
// Provides: {"impl_432"}
// Dependencies: {}
impl Display for RuleError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { for (idx , loc) in self . locations . iter () . enumerate () { if idx == 0 { write ! (f , "[") ? ; } else { write ! (f , ", ") ? ; } write ! (f , "{}:{}" , loc . line , loc . column) ? ; if idx == self . locations . len () - 1 { write ! (f , "] ") ? ; } } write ! (f , "{}" , self . message) ? ; Ok (()) } }
};
}
