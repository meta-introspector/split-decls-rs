macro_rules! deps {
    () => {
        Url!();
        Push!();
        Note!();
        Repository!();
        Fetch!();
        Remote!();
        Name!();
        Direction!();
    };
}

macro_rules! impl_972 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'repo > Remote < 'repo > { # [doc = " Return the name of this remote or `None` if it wasn't persisted to disk yet."] pub fn name (& self) -> Option < & remote :: Name < 'static > > { self . name . as_ref () } # [doc = " Return our repository reference."] pub fn repo (& self) -> & 'repo crate :: Repository { self . repo } # [doc = " Return the set of ref-specs used for `direction`, which may be empty, in order of occurrence in the configuration."] pub fn refspecs (& self , direction : remote :: Direction) -> & [RefSpec] { match direction { remote :: Direction :: Fetch => & self . fetch_specs , remote :: Direction :: Push => & self . push_specs , } } # [doc = " Return how we handle tags when fetching the remote."] pub fn fetch_tags (& self) -> remote :: fetch :: Tags { self . fetch_tags } # [doc = " Return the url used for the given `direction` with rewrites from `url.<base>.insteadOf|pushInsteadOf`, unless the instance"] # [doc = " was created with one of the `_without_url_rewrite()` methods."] # [doc = " For pushing, this is the `remote.<name>.pushUrl` or the `remote.<name>.url` used for fetching, and for fetching it's"] # [doc = " the `remote.<name>.url`."] # [doc = " Note that it's possible to only have the push url set, in which case there will be no way to fetch from the remote as"] # [doc = " the push-url isn't used for that."] pub fn url (& self , direction : remote :: Direction) -> Option < & gix_url :: Url > { match direction { remote :: Direction :: Fetch => self . url_alias . as_ref () . or (self . url . as_ref ()) , remote :: Direction :: Push => self . push_url_alias . as_ref () . or (self . push_url . as_ref ()) . or_else (| | self . url (remote :: Direction :: Fetch)) , } } }
    };
}

impl_972!()