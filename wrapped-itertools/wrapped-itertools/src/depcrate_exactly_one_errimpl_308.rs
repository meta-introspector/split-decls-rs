// Generated macro for impl_308 (impl)
macro_rules! Depcrate_exactly_one_errimpl_308 {
() => {
// Module: crate::exactly_one_err
// Provides: {"impl_308"}
// Dependencies: {}
impl < I > Display for ExactlyOneError < I > where I : Iterator , { fn fmt (& self , f : & mut Formatter) -> FmtResult { let additional = self . additional_len () ; if additional > 0 { write ! (f , "got at least 2 elements when exactly one was expected") } else { write ! (f , "got zero elements when exactly one was expected") } } }
};
}
