// Generated macro for impl_441 (impl)
macro_rules! Depcrate_repository_checkoutimpl_441 {
() => {
// Module: crate::repository::checkout
// Provides: {"impl_441"}
// Dependencies: {}
impl Repository { # [doc = " Return options that can be used to drive a low-level checkout operation."] # [doc = " Use `attributes_source` to determine where `.gitattributes` files should be read from, which depends on"] # [doc = " the presence of a worktree to begin with."] # [doc = " Here, typically this value would be [`gix_worktree::stack::state::attributes::Source::IdMapping`]"] pub fn checkout_options (& self , attributes_source : gix_worktree :: stack :: state :: attributes :: Source ,) -> Result < gix_worktree_state :: checkout :: Options , config :: checkout_options :: Error > { self . config . checkout_options (self , attributes_source) } }
};
}
