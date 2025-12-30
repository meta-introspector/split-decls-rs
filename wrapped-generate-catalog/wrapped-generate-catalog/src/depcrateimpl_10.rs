// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Args { pub fn from_env () -> Result < Option < Self > , Error > { let mut args = Arguments :: from_env () ; if args . contains (["-h" , "--help"]) { println ! ("USAGE: {} [-o FILE] [URL]" , std :: env :: args () . next () . map (Cow :: Owned) . unwrap_or_else (|| "generate-catalog" . into ())) ; return Ok (None) ; } let output = args . opt_value_from_str (["-o" , "--output"]) ? ; let url = args . opt_free_from_str () ? ; Ok (Some (Self { url , output , })) } }
};
}
