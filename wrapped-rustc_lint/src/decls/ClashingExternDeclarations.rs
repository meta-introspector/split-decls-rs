macro_rules! ClashingExternDeclarations {
    () => {
        struct ClashingExternDeclarations { # [doc = " Map of function symbol name to the first-seen hir id for that symbol name.. If seen_decls"] # [doc = " contains an entry for key K, it means a symbol with name K has been seen by this lint and"] # [doc = " the symbol should be reported as a clashing declaration."] seen_decls : UnordMap < Symbol , hir :: OwnerId > , }
    };
}

ClashingExternDeclarations!();