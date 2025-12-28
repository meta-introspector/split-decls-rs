macro_rules! UnresolvedImport {
    () => {
        # [derive (Debug)] pub struct UnresolvedImport { pub decl : InFile < AstPtr < ast :: UseTree > > , }
    };
}

UnresolvedImport!();