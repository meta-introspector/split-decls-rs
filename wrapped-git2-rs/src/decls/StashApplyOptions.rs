macro_rules! deps {
    () => {
        StashApplyProgressCb!();
        CheckoutBuilder!();
    };
}

macro_rules! StashApplyOptions {
    () => {
        deps!();
        # [doc = " Stash application options structure"] pub struct StashApplyOptions < 'cb > { progress : Option < Box < StashApplyProgressCb < 'cb > > > , checkout_options : Option < CheckoutBuilder < 'cb > > , raw_opts : raw :: git_stash_apply_options , }
    };
}

StashApplyOptions!();