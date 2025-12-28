macro_rules! deps {
    () => {
        Path!();
        Default!();
        Options!();
        Error!();
        Cache!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Options { # [cfg (feature = "blob-diff")] pub (crate) fn from_configuration (config : & crate :: config :: Cache) -> Result < Self , options :: init :: Error > { Ok (Options { location : Some (Location :: Path) , rewrites : { let (rewrites , is_configured) = config . diff_renames () ? ; if is_configured { rewrites } else { Some (Default :: default ()) } } , }) } }
    };
}

impl_445!()