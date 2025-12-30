// Generated macro for validate (function)
macro_rules! Depcrate_referencevalidate {
() => {
// Module: crate::reference
// Provides: {"validate"}
// Dependencies: {}
fn validate (path : & BStr , mode : Mode) -> Result < Option < BString > , name :: Error > { let out = crate :: tag :: name_inner (path , match mode { Mode :: Complete | Mode :: Partial => crate :: tag :: Mode :: Validate , Mode :: PartialSanitize => crate :: tag :: Mode :: Sanitize , } ,) ? ; if let Mode :: Complete = mode { let input = out . as_ref () . map_or (path , | b | b . as_bstr ()) ; let saw_slash = input . find_byte (b'/') . is_some () ; if ! saw_slash && ! input . iter () . all (| c | c . is_ascii_uppercase () || * c == b'_') { return Err (name :: Error :: SomeLowercase) ; } } Ok (out) }
};
}
