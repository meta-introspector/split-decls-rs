macro_rules! deps {
    () => {
        StashApplyProgressCb!();
        CheckoutBuilder!();
        StashApplyOptions!();
        StashApplyProgress!();
    };
}

macro_rules! impl_737 {
    () => {
        deps!();
        impl < 'cb > StashApplyOptions < 'cb > { # [doc = " Creates a default set of merge options."] pub fn new () -> StashApplyOptions < 'cb > { let mut opts = StashApplyOptions { progress : None , checkout_options : None , raw_opts : unsafe { mem :: zeroed () } , } ; assert_eq ! (unsafe { raw :: git_stash_apply_init_options (& mut opts . raw_opts , 1) } , 0) ; opts } # [doc = " Set stash application flag to GIT_STASH_APPLY_REINSTATE_INDEX"] pub fn reinstantiate_index (& mut self) -> & mut StashApplyOptions < 'cb > { self . raw_opts . flags = raw :: GIT_STASH_APPLY_REINSTATE_INDEX as u32 ; self } # [doc = " Options to use when writing files to the working directory"] pub fn checkout_options (& mut self , opts : CheckoutBuilder < 'cb >) -> & mut StashApplyOptions < 'cb > { self . checkout_options = Some (opts) ; self } # [doc = " Optional callback to notify the consumer of application progress."] # [doc = ""] # [doc = " Return `true` to continue processing, or `false` to"] # [doc = " abort the stash application."] pub fn progress_cb < C > (& mut self , callback : C) -> & mut StashApplyOptions < 'cb > where C : FnMut (StashApplyProgress) -> bool + 'cb , { self . progress = Some (Box :: new (callback) as Box < StashApplyProgressCb < 'cb > >) ; self . raw_opts . progress_cb = Some (stash_apply_progress_cb) ; self . raw_opts . progress_payload = self as * mut _ as * mut _ ; self } # [doc = " Pointer to a raw git_stash_apply_options"] pub fn raw (& mut self) -> & raw :: git_stash_apply_options { unsafe { if let Some (opts) = self . checkout_options . as_mut () { opts . configure (& mut self . raw_opts . checkout_options) ; } } & self . raw_opts } }
    };
}

impl_737!();