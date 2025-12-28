macro_rules! UnresolvedExternCrate {
    () => {
        # [derive (Debug)] pub struct UnresolvedExternCrate { pub decl : InFile < AstPtr < ast :: ExternCrate > > , }
    };
}

UnresolvedExternCrate!()