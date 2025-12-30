// Generated macro for impl_328 (impl)
macro_rules! Depcrate_repository_configimpl_328 {
() => {
// Module: crate::repository::config
// Provides: {"impl_328"}
// Dependencies: {}
impl Filter { fn new (input : BString) -> Self { match input . try_as_key () { Some (key) => Filter { name : key . section_name . into () , subsection : key . subsection_name . map (ToOwned :: to_owned) , } , None => Filter { name : input . to_string () , subsection : None , } , } } fn matches_section (& self , section : & gix :: config :: file :: Section < '_ >) -> bool { let ignore_case = gix :: glob :: wildmatch :: Mode :: IGNORE_CASE ; if ! gix :: glob :: wildmatch (self . name . as_bytes () . into () , section . header () . name () , ignore_case) { return false ; } match (self . subsection . as_deref () , section . header () . subsection_name ()) { (Some (filter) , Some (name)) => { if ! gix :: glob :: wildmatch (filter . as_slice () . into () , name , ignore_case) { return false ; } } (None , _) => { } (Some (_) , None) => return false , } true } }
};
}
