macro_rules! deps {
    () => {
        Clone!();
    };
}

macro_rules! DryRun {
    () => {
        deps!();
        # [doc = " If `Yes`, don't really make changes but do as much as possible to get an idea of what would be done."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client"))] pub (crate) enum DryRun { # [doc = " Enable dry-run mode and don't actually change the underlying repository in any way."] Yes , # [doc = " Run the operation like normal, making changes to the underlying repository."] No , }
    };
}

DryRun!();