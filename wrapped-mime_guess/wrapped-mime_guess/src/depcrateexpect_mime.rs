// Generated macro for expect_mime (function)
macro_rules! Depcrateexpect_mime {
() => {
// Module: crate
// Provides: {"expect_mime"}
// Dependencies: {}
fn expect_mime (s : & str) -> Mime { s . parse () . unwrap_or_else (| e | panic ! ("failed to parse media-type {:?}: {}" , s , e)) }
};
}
