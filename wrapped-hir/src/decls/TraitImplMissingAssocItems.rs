macro_rules! deps {
    () => {
        Impl!();
        AssocItem!();
    };
}

macro_rules! TraitImplMissingAssocItems {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct TraitImplMissingAssocItems { pub file_id : HirFileId , pub impl_ : AstPtr < ast :: Impl > , pub missing : Vec < (Name , AssocItem) > , }
    };
}

TraitImplMissingAssocItems!();