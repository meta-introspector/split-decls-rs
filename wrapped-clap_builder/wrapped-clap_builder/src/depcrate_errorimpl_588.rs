// Generated macro for impl_588 (impl)
macro_rules! Depcrate_errorimpl_588 {
() => {
// Module: crate::error
// Provides: {"impl_588"}
// Dependencies: {}
impl < F : ErrorFormatter > Display for Error < F > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { ok ! (write ! (f , "{}" , self . formatted ())) ; if let Some (backtrace) = self . inner . backtrace . as_ref () { ok ! (writeln ! (f)) ; ok ! (writeln ! (f , "Backtrace:")) ; ok ! (writeln ! (f , "{backtrace}")) ; } Ok (()) } }
};
}
