macro_rules! deps {
    () => {
        SubmoduleIgnore!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl SubmoduleIgnore { # [doc = " Converts a [`raw::git_submodule_ignore_t`] to a [`SubmoduleIgnore`]"] pub fn from_raw (raw : raw :: git_submodule_ignore_t) -> Self { match raw { raw :: GIT_SUBMODULE_IGNORE_UNSPECIFIED => SubmoduleIgnore :: Unspecified , raw :: GIT_SUBMODULE_IGNORE_NONE => SubmoduleIgnore :: None , raw :: GIT_SUBMODULE_IGNORE_UNTRACKED => SubmoduleIgnore :: Untracked , raw :: GIT_SUBMODULE_IGNORE_DIRTY => SubmoduleIgnore :: Dirty , raw :: GIT_SUBMODULE_IGNORE_ALL => SubmoduleIgnore :: All , n => panic ! ("unknown submodule ignore rule: {}" , n) , } } }
    };
}

impl_105!()