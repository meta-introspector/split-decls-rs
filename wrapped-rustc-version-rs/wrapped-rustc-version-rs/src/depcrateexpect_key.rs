// Generated macro for expect_key (function)
macro_rules! Depcrateexpect_key {
() => {
// Module: crate
// Provides: {"expect_key"}
// Dependencies: {}
fn expect_key (key : & str , map : & HashMap < & str , & str >) -> Result < String , Error > { map . get (key) . map (| & v | String :: from (v)) . ok_or (Error :: UnexpectedVersionFormat) }
};
}
