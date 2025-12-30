// Generated macro for fail (function)
macro_rules! Depcratefail {
() => {
// Module: crate
// Provides: {"fail"}
// Dependencies: {}
fn fail (s : & str) -> ! { panic ! ("\n{}\n\nbuild script failed, must exit now" , s) }
};
}
