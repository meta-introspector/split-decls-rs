macro_rules! LibraryRoots {
    () => {
        # [doc = " The set of roots for crates.io libraries."] # [doc = " Files in libraries are assumed to never change."] # [salsa :: input (singleton , debug)] pub struct LibraryRoots { # [returns (ref)] pub roots : FxHashSet < SourceRootId > , }
    };
}

LibraryRoots!()