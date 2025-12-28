macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Default for Options < '_ > { fn default () -> Self { Options { required_trust : gix_sec :: Trust :: Reduced , ceiling_dirs : vec ! [] , match_ceiling_dir_or_error : true , cross_fs : false , dot_git_only : false , current_dir : None , } } }
    };
}

impl_15!()