// Generated macro for StashApplyOptions (struct)
macro_rules! Depcrate_stashStashApplyOptions {
() => {
// Module: crate::stash
// Provides: {"StashApplyOptions"}
// Dependencies: {}
# [doc = " Stash application options structure"] pub struct StashApplyOptions < 'cb > { progress : Option < Box < StashApplyProgressCb < 'cb > > > , checkout_options : Option < CheckoutBuilder < 'cb > > , raw_opts : raw :: git_stash_apply_options , }
};
}
