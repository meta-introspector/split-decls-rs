macro_rules! deps {
    () => {
        FnRetTy!();
        TyKind!();
        FnDecl!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl < 'hir > FnDecl < 'hir > { pub fn opt_delegation_sig_id (& self) -> Option < DefId > { if let FnRetTy :: Return (ty) = self . output && let TyKind :: InferDelegation (sig_id , _) = ty . kind { return Some (sig_id) ; } None } }
    };
}

impl_287!()