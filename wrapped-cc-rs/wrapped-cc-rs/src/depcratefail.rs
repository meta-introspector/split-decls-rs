// Generated macro for fail (function)
macro_rules! Depcratefail {
() => {
// Module: crate
// Provides: {"fail"}
// Dependencies: {}
fn fail (s : & str) -> ! { eprintln ! ("\n\nerror occurred in cc-rs: {s}\n\n") ; std :: process :: exit (1) ; }
};
}
