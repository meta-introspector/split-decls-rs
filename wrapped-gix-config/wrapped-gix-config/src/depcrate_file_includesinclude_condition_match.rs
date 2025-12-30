// Generated macro for include_condition_match (function)
macro_rules! Depcrate_file_includesinclude_condition_match {
() => {
// Module: crate::file::includes
// Provides: {"include_condition_match"}
// Dependencies: {}
fn include_condition_match (condition : & BStr , target_config_path : Option < & Path > , search_config : & File < 'static > , options : Options < '_ > ,) -> Result < bool , Error > { let mut tokens = condition . splitn (2 , | b | * b == b':') ; let (prefix , condition) = match (tokens . next () , tokens . next ()) { (Some (a) , Some (b)) => (a , b) , _ => return Ok (false) , } ; let condition = condition . as_bstr () ; match prefix { b"gitdir" => gitdir_matches (condition , target_config_path , options , gix_glob :: wildmatch :: Mode :: empty () ,) , b"gitdir/i" => gitdir_matches (condition , target_config_path , options , gix_glob :: wildmatch :: Mode :: IGNORE_CASE ,) , b"onbranch" => Ok (onbranch_matches (condition , options . conditional) . is_some ()) , b"hasconfig" => { let mut tokens = condition . splitn (2 , | b | * b == b':') ; let (key_glob , value_glob) = match (tokens . next () , tokens . next ()) { (Some (a) , Some (b)) => (a , b) , _ => return Ok (false) , } ; if key_glob . as_bstr () != "remote.*.url" { return Ok (false) ; } let Some (sections) = search_config . sections_by_name ("remote") else { return Ok (false) ; } ; for remote in sections { for url in remote . values ("url") { let glob_matches = gix_glob :: wildmatch (value_glob . as_bstr () , url . as_ref () , gix_glob :: wildmatch :: Mode :: NO_MATCH_SLASH_LITERAL ,) ; if glob_matches { return Ok (true) ; } } } Ok (false) } _ => Ok (false) , } }
};
}
