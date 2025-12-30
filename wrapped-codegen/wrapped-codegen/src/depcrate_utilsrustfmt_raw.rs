// Generated macro for rustfmt_raw (function)
macro_rules! Depcrate_utilsrustfmt_raw {
() => {
// Module: crate::utils
// Provides: {"rustfmt_raw"}
// Dependencies: {}
fn rustfmt_raw (s : & str) -> anyhow :: Result < String > { let child = Command :: new ("rustfmt") . args (["--config" , "normalize_doc_attributes=true"]) . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . stderr (Stdio :: piped ()) . spawn () ? ; child . stdin . as_ref () . unwrap () . write_all (s . as_bytes ()) ? ; let o = child . wait_with_output () ? ; if o . status . success () { Ok (std :: str :: from_utf8 (& o . stdout) ? . to_string ()) } else { bail ! ("{}" , std :: str :: from_utf8 (& o . stderr) ?) ; } }
};
}
