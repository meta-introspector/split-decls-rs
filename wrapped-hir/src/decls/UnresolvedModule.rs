macro_rules! deps {
    () => {
        Module!();
    };
}

macro_rules! UnresolvedModule {
    () => {
        deps!();
        # [derive (Debug)] pub struct UnresolvedModule { pub decl : InFile < AstPtr < ast :: Module > > , pub candidates : Box < [String] > , }
    };
}

UnresolvedModule!();