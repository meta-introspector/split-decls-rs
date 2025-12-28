macro_rules! deps {
    () => {
        Clone!();
        Repository!();
        Options!();
    };
}

macro_rules! Editor {
    () => {
        deps!();
        # [doc = " All state needed to conveniently edit a tree, using only [update-or-insert](Editor::upsert()) and [removals](Editor::remove())."] # [cfg (feature = "tree-editor")] # [derive (Clone)] pub struct Editor < 'repo > { pub (crate) inner : gix_object :: tree :: Editor < 'repo > , pub (crate) validate : gix_validate :: path :: component :: Options , # [doc = " The owning repository."] pub repo : & 'repo crate :: Repository , }
    };
}

Editor!();