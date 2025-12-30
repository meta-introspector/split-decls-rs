// Generated macro for collect_meta_vars (function)
macro_rules! Depcrate_quotecollect_meta_vars {
() => {
// Module: crate::quote
// Provides: {"collect_meta_vars"}
// Dependencies: {}
# [doc = " Helper function to support macro repetitions like `$( CONTENTS ) SEP_OPT *` in `quote!`."] # [doc = " Recursively collects all `Ident`s (meta-variables) that follow a `$`"] # [doc = " from the given `CONTENTS` stream, preserving their order of appearance."] fn collect_meta_vars (content_stream : TokenStream) -> Vec < Ident > { fn helper (stream : TokenStream , out : & mut Vec < Ident >) { let mut iter = stream . into_iter () . peekable () ; while let Some (tree) = iter . next () { match & tree { TokenTree :: Punct (tt) if tt . as_char () == '$' => { if let Some (TokenTree :: Ident (id)) = iter . peek () { out . push (id . clone ()) ; iter . next () ; } } TokenTree :: Group (tt) => { helper (tt . stream () , out) ; } _ => { } } } } let mut vars = Vec :: new () ; helper (content_stream , & mut vars) ; vars }
};
}
