// Generated macro for escape_impl (macro)
macro_rules! Depcrate_encode_html_entityescape_impl {
() => {
// Module: crate::encode::html_entity
// Provides: {"escape_impl"}
// Dependencies: {}
macro_rules ! escape_impl { (@ inner [$ dollar : tt] $ name : ident ; $ ($ l : expr => $ r : expr) ,+ $ (,) *) => { macro_rules ! $ name { ($ dollar e : expr) => { match $ dollar e { $ ($ l => break $ r ,) + _ => () , } } ; (vec $ dollar e : expr , $ dollar v : ident , $ dollar b : ident , $ dollar start : ident , $ dollar end : ident) => { match $ dollar e { $ ($ l => { $ dollar v . extend_from_slice (&$ dollar b [$ dollar start ..$ dollar end]) ; $ dollar start = $ dollar end + 1 ; $ dollar v . extend_from_slice ($ r) ; }) + _ => () , } $ dollar end += 1 ; } ; (writer $ dollar e : expr , $ dollar w : ident , $ dollar b : ident , $ dollar start : ident , $ dollar end : ident) => { match $ dollar e { $ ($ l => { $ dollar w . write_all (&$ dollar b [$ dollar start ..$ dollar end]) ?; $ dollar start = $ dollar end + 1 ; $ dollar w . write_all ($ r) ?; }) + _ => () , } $ dollar end += 1 ; } ; } } ; ($ name : ident ; $ ($ l : expr => $ r : expr) ,+ $ (,) *) => { escape_impl ! { @ inner [$] $ name ; $ ($ l => $ r . as_ref () ,) * } } ; }
};
}
