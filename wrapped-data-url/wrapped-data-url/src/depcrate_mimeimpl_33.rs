// Generated macro for impl_33 (impl)
macro_rules! Depcrate_mimeimpl_33 {
() => {
// Module: crate::mime
// Provides: {"impl_33"}
// Dependencies: {}
# [doc = " <https://mimesniff.spec.whatwg.org/#parsing-a-mime-type>"] impl FromStr for Mime { type Err = MimeParsingError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { parse (s) . ok_or (MimeParsingError (())) } }
};
}
