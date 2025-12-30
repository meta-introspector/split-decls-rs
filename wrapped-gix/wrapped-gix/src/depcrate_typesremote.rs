// Generated macro for Remote (struct)
macro_rules! Depcrate_typesRemote {
() => {
// Module: crate::types
// Provides: {"Remote"}
// Dependencies: {}
# [doc = " A remote which represents a way to interact with hosts for remote clones of the parent repository."] # [derive (Debug , Clone , PartialEq)] pub struct Remote < 'repo > { # [doc = " The remotes symbolic name, only present if persisted in git configuration files."] pub (crate) name : Option < remote :: Name < 'static > > , # [doc = " The url of the host to talk to, after application of replacements. If it is unset, the `push_url` must be set."] # [doc = " and fetches aren't possible."] pub (crate) url : Option < gix_url :: Url > , # [doc = " The rewritten `url`, if it was rewritten."] pub (crate) url_alias : Option < gix_url :: Url > , # [doc = " The url to use for pushing specifically."] pub (crate) push_url : Option < gix_url :: Url > , # [doc = " The rewritten `push_url`, if it was rewritten."] pub (crate) push_url_alias : Option < gix_url :: Url > , # [doc = " Refspecs for use when fetching."] pub (crate) fetch_specs : Vec < gix_refspec :: RefSpec > , # [doc = " Refspecs for use when pushing."] pub (crate) push_specs : Vec < gix_refspec :: RefSpec > , # [doc = " Tell us what to do with tags when fetched."] pub (crate) fetch_tags : remote :: fetch :: Tags , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
};
}
