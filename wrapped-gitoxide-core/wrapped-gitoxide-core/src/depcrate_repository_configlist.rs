// Generated macro for list (function)
macro_rules! Depcrate_repository_configlist {
() => {
// Module: crate::repository::config
// Provides: {"list"}
// Dependencies: {}
pub fn list (repo : gix :: Repository , filters : Vec < BString > , overrides : Vec < BString > , format : OutputFormat , mut out : impl std :: io :: Write ,) -> Result < () > { if format != OutputFormat :: Human { bail ! ("Only human output format is supported at the moment") ; } let repo = gix :: open_opts (repo . git_dir () , repo . open_options () . clone () . cli_overrides (overrides)) ? ; let config = repo . config_snapshot () ; if let Some (frontmatter) = config . frontmatter () { for event in frontmatter { event . write_to (& mut out) ? ; } } let filters : Vec < _ > = filters . into_iter () . map (Filter :: new) . collect () ; let mut last_meta = None ; let mut it = config . sections_and_postmatter () . peekable () ; while let Some ((section , matter)) = it . next () { if ! filters . is_empty () && ! filters . iter () . any (| filter | filter . matches_section (section)) { continue ; } let meta = section . meta () ; if last_meta != Some (meta) { write_meta (meta , & mut out) ? ; } last_meta = Some (meta) ; section . write_to (& mut out) ? ; for event in matter { event . write_to (& mut out) ? ; } if it . peek () . is_some_and (| (next_section , _) | next_section . header () . name () != section . header () . name ()) { writeln ! (& mut out) ? ; } } Ok (()) }
};
}
