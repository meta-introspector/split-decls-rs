macro_rules! deps {
    () => {
        Error!();
        Editor!();
        Tree!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < 'repo > super :: Editor < 'repo > { # [doc = " Initialize a new editor from the given `tree`."] pub fn new (tree : & crate :: Tree < 'repo >) -> Result < Self , init :: Error > { let tree_ref = tree . decode () ? ; let repo = tree . repo ; let validate = repo . config . protect_options () ? ; Ok (super :: Editor { inner : gix_object :: tree :: Editor :: new (tree_ref . into () , & repo . objects , repo . object_hash ()) , validate , repo , }) } # [doc = " Detach all extras and return the underlying plumbing editor, which won't perform validation"] # [doc = " when writing the tree."] pub fn detach (self) -> gix_object :: tree :: Editor < 'repo > { self . inner } }
    };
}

impl_190!();