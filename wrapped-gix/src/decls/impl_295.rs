macro_rules! deps {
    () => {
        Error!();
        Repository!();
        Options!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl Repository { # [doc = " Return options that can be used to drive a low-level checkout operation."] # [doc = " Use `attributes_source` to determine where `.gitattributes` files should be read from, which depends on"] # [doc = " the presence of a worktree to begin with."] # [doc = " Here, typically this value would be [`gix_worktree::stack::state::attributes::Source::IdMapping`]"] pub fn checkout_options (& self , attributes_source : gix_worktree :: stack :: state :: attributes :: Source ,) -> Result < gix_worktree_state :: checkout :: Options , config :: checkout_options :: Error > { self . config . checkout_options (self , attributes_source) } }
    };
}

impl_295!();