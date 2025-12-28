macro_rules! deps {
    () => {
        RevertOptions!();
        MergeOptions!();
        CheckoutBuilder!();
    };
}

macro_rules! impl_704 {
    () => {
        deps!();
        impl < 'cb > RevertOptions < 'cb > { # [doc = " Creates a default set of revert options"] pub fn new () -> RevertOptions < 'cb > { RevertOptions { mainline : 0 , checkout_builder : None , merge_opts : None , } } # [doc = " Set the mainline value"] # [doc = ""] # [doc = " For merge commits, the \"mainline\" is treated as the parent."] pub fn mainline (& mut self , mainline : u32) -> & mut Self { self . mainline = mainline ; self } # [doc = " Set the checkout builder"] pub fn checkout_builder (& mut self , cb : CheckoutBuilder < 'cb >) -> & mut Self { self . checkout_builder = Some (cb) ; self } # [doc = " Set the merge options"] pub fn merge_opts (& mut self , merge_opts : MergeOptions) -> & mut Self { self . merge_opts = Some (merge_opts) ; self } # [doc = " Obtain the raw struct"] pub fn raw (& mut self) -> raw :: git_revert_options { unsafe { let mut checkout_opts : raw :: git_checkout_options = mem :: zeroed () ; raw :: git_checkout_init_options (& mut checkout_opts , raw :: GIT_CHECKOUT_OPTIONS_VERSION) ; if let Some (ref mut cb) = self . checkout_builder { cb . configure (& mut checkout_opts) ; } let mut merge_opts : raw :: git_merge_options = mem :: zeroed () ; raw :: git_merge_init_options (& mut merge_opts , raw :: GIT_MERGE_OPTIONS_VERSION) ; if let Some (ref opts) = self . merge_opts { ptr :: copy (opts . raw () , & mut merge_opts , 1) ; } let mut revert_opts : raw :: git_revert_options = mem :: zeroed () ; raw :: git_revert_options_init (& mut revert_opts , raw :: GIT_REVERT_OPTIONS_VERSION) ; revert_opts . mainline = self . mainline ; revert_opts . checkout_opts = checkout_opts ; revert_opts . merge_opts = merge_opts ; revert_opts } } }
    };
}

impl_704!()