// Generated macro for rewrite_url (function)
macro_rules! Depcrate_remote_initrewrite_url {
() => {
// Module: crate::remote::init
// Provides: {"rewrite_url"}
// Dependencies: {}
pub (crate) fn rewrite_url (config : & config :: Cache , url : Option < & gix_url :: Url > , direction : remote :: Direction ,) -> Result < Option < gix_url :: Url > , Error > { url . and_then (| url | config . url_rewrite () . longest (url , direction)) . map (| url | { gix_url :: parse (url . as_ref ()) . map_err (| err | Error :: RewrittenUrlInvalid { kind : match direction { remote :: Direction :: Fetch => "fetch" , remote :: Direction :: Push => "push" , } , source : err , rewritten_url : url , }) }) . transpose () }
};
}
