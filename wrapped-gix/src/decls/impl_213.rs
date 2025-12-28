macro_rules! deps {
    () => {
        Error!();
        Options!();
        Tree!();
        Platform!();
        Note!();
        Default!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        # [doc = " Diffing"] impl < 'repo > Tree < 'repo > { # [doc = " Return a platform to see the changes needed to create other trees, for instance."] # [doc = ""] # [doc = " # Performance"] # [doc = ""] # [doc = " It's highly recommended to [set an object cache](crate::Repository::compute_object_cache_size_for_tree_diffs)"] # [doc = " to avoid extracting the same object multiple times."] # [doc = " By default, similar to `git diff`, rename tracking will be enabled if it is not configured."] # [doc = ""] # [doc = " Note that if a clone with `--filter=blob=none` was created, rename tracking may fail as it might"] # [doc = " try to access blobs to compute a similarity metric. Thus, it's more compatible to turn rewrite tracking off"] # [doc = " using [`Options::track_rewrites()`](crate::diff::Options::track_rewrites())."] # [allow (clippy :: result_large_err)] # [doc (alias = "diff_tree_to_tree" , alias = "git2")] pub fn changes < 'a > (& 'a self) -> Result < Platform < 'a , 'repo > , crate :: diff :: options :: init :: Error > { Ok (Platform { state : Default :: default () , lhs : self , options : crate :: diff :: Options :: from_configuration (& self . repo . config) ? , }) } }
    };
}

impl_213!()