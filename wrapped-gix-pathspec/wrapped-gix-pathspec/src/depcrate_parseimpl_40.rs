// Generated macro for impl_40 (impl)
macro_rules! Depcrate_parseimpl_40 {
() => {
// Module: crate::parse
// Provides: {"impl_40"}
// Dependencies: {}
impl Pattern { # [doc = " Try to parse a path-spec pattern from the given `input` bytes."] pub fn from_bytes (input : & [u8] , Defaults { signature , search_mode , literal , } : Defaults ,) -> Result < Self , Error > { if input . is_empty () { return Err (Error :: EmptyString) ; } if literal { return Ok (Self :: from_literal (input , signature)) ; } if input . as_bstr () == ":" { return Ok (Pattern { nil : true , .. Default :: default () }) ; } let mut p = Pattern { signature , search_mode : SearchMode :: default () , .. Default :: default () } ; let mut cursor = 0 ; if input . first () == Some (& b':') { cursor += 1 ; p . signature |= parse_short_keywords (input , & mut cursor) ? ; if let Some (b'(') = input . get (cursor) { cursor += 1 ; parse_long_keywords (input , & mut p , & mut cursor) ? ; } } if search_mode != Default :: default () && p . search_mode == Default :: default () { p . search_mode = search_mode ; } let mut path = & input [cursor ..] ; if path . last () == Some (& b'/') { p . signature |= MagicSignature :: MUST_BE_DIR ; path = & path [.. path . len () - 1] ; } p . path = path . into () ; Ok (p) } # [doc = " Take `input` literally without parsing anything. This will also set our mode to `literal` to allow this pathspec to match `input` verbatim, and"] # [doc = " use `default_signature` as magic signature."] pub fn from_literal (input : & [u8] , default_signature : MagicSignature) -> Self { Pattern { path : input . into () , signature : default_signature , search_mode : SearchMode :: Literal , .. Default :: default () } } }
};
}
