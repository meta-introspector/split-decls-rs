macro_rules! deps {
    () => {
        Impl!();
        Trait!();
        AssocItem!();
    };
}

macro_rules! TraitImplRedundantAssocItems {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct TraitImplRedundantAssocItems { pub file_id : HirFileId , pub trait_ : Trait , pub impl_ : AstPtr < ast :: Impl > , pub assoc_item : (Name , AssocItem) , }
    };
}

TraitImplRedundantAssocItems!();