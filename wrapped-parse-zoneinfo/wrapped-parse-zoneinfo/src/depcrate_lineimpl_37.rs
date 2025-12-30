// Generated macro for impl_37 (impl)
macro_rules! Depcrate_lineimpl_37 {
() => {
// Module: crate::line
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a > Zone < 'a > { fn from_str (input : & 'a str) -> Result < Self , Error > { let mut iter = input . split_ascii_whitespace () ; if iter . next () != Some ("Zone") { return Err (Error :: NotParsedAsZoneLine) ; } let name = match iter . next () { Some (name) => name , None => return Err (Error :: NotParsedAsZoneLine) , } ; Ok (Self { name , info : ZoneInfo :: from_iter (iter) ? , }) } }
};
}
