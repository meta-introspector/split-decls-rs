// Generated macro for build_uuid (function)
macro_rules! Depcratebuild_uuid {
() => {
// Module: crate
// Provides: {"build_uuid"}
// Dependencies: {}
fn build_uuid (input : TokenStream) -> Result < TokenStream , Error > { let str_lit = match syn :: parse :: < syn :: Lit > (input) { Ok (syn :: Lit :: Str (literal)) => literal , _ => return Err (Error :: NonStringLiteral) , } ; let bytes = parser :: try_parse (& str_lit . value ()) . map_err (| e | Error :: UuidParse (str_lit , e . into_err ())) ? ; let tokens = bytes . iter () . map (| byte | quote ! { # byte , }) . collect :: < TokenStream2 > () ; Ok (quote ! { [# tokens] } . into ()) }
};
}
