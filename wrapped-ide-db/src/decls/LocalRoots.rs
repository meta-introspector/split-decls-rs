macro_rules! LocalRoots {
    () => {
        # [doc = " The set of \"local\" (that is, from the current workspace) roots."] # [doc = " Files in local roots are assumed to change frequently."] # [salsa :: input (singleton , debug)] pub struct LocalRoots { # [returns (ref)] pub roots : FxHashSet < SourceRootId > , }
    };
}

LocalRoots!()