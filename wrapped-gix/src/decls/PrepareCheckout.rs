macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! PrepareCheckout {
    () => {
        deps!();
        # [doc = " A utility to collect configuration on how to perform a checkout into a working tree,"] # [doc = " and when dropped without checking out successfully the fetched repository will be deleted from disk."] # [must_use] # [cfg (feature = "worktree-mutation")] # [derive (Debug)] pub struct PrepareCheckout { # [doc = " A freshly initialized repository which is owned by us, or `None` if it was successfully checked out."] pub (self) repo : Option < crate :: Repository > , # [doc = " The name of the reference to check out. If `None`, the reference pointed to by `HEAD` will be checked out."] pub (self) ref_name : Option < gix_ref :: PartialName > , }
    };
}

PrepareCheckout!()