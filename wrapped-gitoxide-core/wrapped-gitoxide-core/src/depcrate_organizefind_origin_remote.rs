// Generated macro for find_origin_remote (function)
macro_rules! Depcrate_organizefind_origin_remote {
() => {
// Module: crate::organize
// Provides: {"find_origin_remote"}
// Dependencies: {}
fn find_origin_remote (repo : & Path) -> anyhow :: Result < Option < gix_url :: Url > > { let non_bare = repo . join (".git") . join ("config") ; let local = gix :: config :: Source :: Local ; let config = gix :: config :: File :: from_path_no_includes (non_bare . as_path () . into () , local) . or_else (| _ | gix :: config :: File :: from_path_no_includes (repo . join ("config") , local)) ? ; Ok (config . string ("remote.origin.url") . map (| url | gix_url :: Url :: from_bytes (url . as_ref ())) . transpose () ?) }
};
}
