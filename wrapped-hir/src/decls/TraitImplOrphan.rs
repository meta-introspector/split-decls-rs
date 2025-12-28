macro_rules! deps {
    () => {
        Impl!();
    };
}

macro_rules! TraitImplOrphan {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct TraitImplOrphan { pub file_id : HirFileId , pub impl_ : AstPtr < ast :: Impl > , }
    };
}

TraitImplOrphan!();