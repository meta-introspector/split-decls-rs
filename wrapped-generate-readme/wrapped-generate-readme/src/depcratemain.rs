// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let output = Command :: new ("cargo") . args (["readme" , "--no-license"]) . output () . unwrap () ; let readme = String :: from_utf8 (output . stdout) . unwrap () ; let body = Regex :: new (r"\[(`[^`]*`)\]") . unwrap () . replace_all (& readme , | caps : & Captures | caps [1] . to_string ()) ; println ! ("{}\n\n{}\n{}" , COPYRIGHT_HEADER , body , DISCLAIMER_FOOTER) ; }
};
}
