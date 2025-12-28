macro_rules! AnnNode {
    () => {
        pub enum AnnNode < 'a > { Name (& 'a Symbol) , Block (& 'a hir :: Block < 'a >) , Item (& 'a hir :: Item < 'a >) , SubItem (HirId) , Expr (& 'a hir :: Expr < 'a >) , Pat (& 'a hir :: Pat < 'a >) , TyPat (& 'a hir :: TyPat < 'a >) , Arm (& 'a hir :: Arm < 'a >) , }
    };
}

AnnNode!()