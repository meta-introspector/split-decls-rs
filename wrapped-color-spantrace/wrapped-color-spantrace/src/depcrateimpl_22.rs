// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl fmt :: Display for ColorSpanTrace < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut err = Ok (()) ; let mut span = 0 ; writeln ! (f , "{:━^80}\n" , " SPANTRACE ") ? ; self . span_trace . with_spans (| metadata , fields | { let frame = Frame { metadata , fields , theme : self . theme , } ; if span > 0 { try_bool ! (write ! (f , "\n" ,) , err) ; } try_bool ! (frame . print (span , f) , err) ; if Verbosity :: lib_from_env () == Verbosity :: Full { try_bool ! (frame . print_source_if_avail (f) , err) ; } span += 1 ; true }) ; err } }
};
}
