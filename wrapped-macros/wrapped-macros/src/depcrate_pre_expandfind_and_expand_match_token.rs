// Generated macro for find_and_expand_match_token (function)
macro_rules! Depcrate_pre_expandfind_and_expand_match_token {
() => {
// Module: crate::pre_expand
// Provides: {"find_and_expand_match_token"}
// Dependencies: {}
fn find_and_expand_match_token (cx : & mut ext :: base :: ExtCtxt , tts : Vec < ast :: TokenTree >) -> Vec < ast :: TokenTree > { let mut expanded = Vec :: new () ; let mut tts = tts . into_iter () . peekable () ; while let Some (tt) = tts . next () { match tt { ast :: TokenTree :: Token (span , token :: Token :: Ident (ident , token :: IdentStyle :: Plain)) if ident . name . as_str () == "match_token" => { if ! matches ! (tts . next () , Some (ast :: TokenTree :: Token (_ , token :: Token :: Not))) { expanded . push (tt) ; continue } match tts . next () { Some (ast :: TokenTree :: Delimited (_ , block)) => { cx . bt_push (expn_info (span)) ; expanded . extend (match match_token :: expand_to_tokens (cx , span , & block . tts) { Ok (tts) => tts , Err ((span , message)) => { cx . parse_sess . span_diagnostic . span_err (span , message) ; panic ! ("Error in match_token! expansion.") ; } }) ; cx . bt_pop () ; } _ => panic ! ("expected a block after {:?}" , span) } } ast :: TokenTree :: Delimited (span , mut block) => { Rc :: make_mut (& mut block) ; let block = Rc :: try_unwrap (block) . unwrap () ; expanded . push (ast :: TokenTree :: Delimited (span , Rc :: new (ast :: Delimited { delim : block . delim , open_span : block . open_span , tts : find_and_expand_match_token (cx , block . tts) , close_span : block . close_span , }))) } _ => expanded . push (tt) } } expanded }
};
}
