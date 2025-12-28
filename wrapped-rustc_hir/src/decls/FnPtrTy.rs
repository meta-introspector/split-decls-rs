macro_rules! deps {
    () => {
        FnDecl!();
        Safety!();
        GenericParam!();
    };
}

macro_rules! FnPtrTy {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct FnPtrTy < 'hir > { pub safety : Safety , pub abi : ExternAbi , pub generic_params : & 'hir [GenericParam < 'hir >] , pub decl : & 'hir FnDecl < 'hir > , pub param_idents : & 'hir [Option < Ident >] , }
    };
}

FnPtrTy!();