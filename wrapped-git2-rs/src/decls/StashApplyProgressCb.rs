macro_rules! deps {
    () => {
        StashApplyProgress!();
    };
}

macro_rules! StashApplyProgressCb {
    () => {
        deps!();
        # [doc = " Stash application progress notification function."] # [doc = ""] # [doc = " Return `true` to continue processing, or `false` to"] # [doc = " abort the stash application."] pub type StashApplyProgressCb < 'a > = dyn FnMut (StashApplyProgress) -> bool + 'a ;
    };
}

StashApplyProgressCb!();