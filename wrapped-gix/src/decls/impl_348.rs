macro_rules! deps {
    () => {
        Submodule!();
        Kind!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl From < gix_discover :: repository :: Kind > for Kind { fn from (v : gix_discover :: repository :: Kind) -> Self { match v { gix_discover :: repository :: Kind :: Submodule { .. } | gix_discover :: repository :: Kind :: SubmoduleGitDir => { Kind :: WorkTree { is_linked : false } } gix_discover :: repository :: Kind :: PossiblyBare => Kind :: Bare , gix_discover :: repository :: Kind :: WorkTreeGitDir { .. } => Kind :: WorkTree { is_linked : true } , gix_discover :: repository :: Kind :: WorkTree { linked_git_dir } => Kind :: WorkTree { is_linked : linked_git_dir . is_some () , } , } } }
    };
}

impl_348!()