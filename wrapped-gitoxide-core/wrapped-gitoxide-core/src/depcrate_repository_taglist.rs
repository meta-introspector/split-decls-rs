// Generated macro for list (function)
macro_rules! Depcrate_repository_taglist {
() => {
// Module: crate::repository::tag
// Provides: {"list"}
// Dependencies: {}
pub fn list (repo : gix :: Repository , out : & mut dyn std :: io :: Write , format : OutputFormat) -> anyhow :: Result < () > { if format != OutputFormat :: Human { anyhow :: bail ! ("JSON output isn't supported") ; } let platform = repo . references () ? ; let mut tags : Vec < _ > = platform . tags () ? . flatten () . map (| mut reference | { let tag = reference . peel_to_tag () ; let tag_ref = tag . as_ref () . map (gix :: Tag :: decode) ; let name = reference . name () . shorten () ; let mut fields = Vec :: new () ; let version = Version :: parse (name) ; match tag_ref { Ok (Ok (tag_ref)) => { fields . push (format ! ("tag name: {}" , if name == tag_ref . name { "*" . into () } else { tag_ref . name })) ; if tag_ref . pgp_signature . is_some () { fields . push ("signed" . into ()) ; } (version , format ! ("{name} [{fields}]" , fields = fields . join (", "))) } _ => (version , name . to_string ()) , } }) . collect () ; tags . sort_by (| a , b | a . 0 . cmp (& b . 0)) ; for (_ , tag) in tags { writeln ! (out , "{tag}") ? ; } Ok (()) }
};
}
