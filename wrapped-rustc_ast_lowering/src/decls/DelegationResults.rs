macro_rules! DelegationResults {
    () => {
        pub (crate) struct DelegationResults < 'hir > { pub body_id : hir :: BodyId , pub sig : hir :: FnSig < 'hir > , pub ident : Ident , pub generics : & 'hir hir :: Generics < 'hir > , }
    };
}

DelegationResults!();