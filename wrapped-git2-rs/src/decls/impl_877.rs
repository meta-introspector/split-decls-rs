macro_rules! deps {
    () => {
        SubmoduleUpdate!();
        Rebase!();
    };
}

macro_rules! impl_877 {
    () => {
        deps!();
        impl SubmoduleUpdate { # [doc = " Converts a [`raw::git_submodule_update_t`] to a [`SubmoduleUpdate`]"] pub fn from_raw (raw : raw :: git_submodule_update_t) -> Self { match raw { raw :: GIT_SUBMODULE_UPDATE_CHECKOUT => SubmoduleUpdate :: Checkout , raw :: GIT_SUBMODULE_UPDATE_REBASE => SubmoduleUpdate :: Rebase , raw :: GIT_SUBMODULE_UPDATE_MERGE => SubmoduleUpdate :: Merge , raw :: GIT_SUBMODULE_UPDATE_NONE => SubmoduleUpdate :: None , raw :: GIT_SUBMODULE_UPDATE_DEFAULT => SubmoduleUpdate :: Default , n => panic ! ("unknown submodule update strategy: {}" , n) , } } }
    };
}

impl_877!()