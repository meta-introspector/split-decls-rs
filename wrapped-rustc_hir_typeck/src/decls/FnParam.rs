macro_rules! FnParam {
    () => {
        # [derive (Clone , Copy)] enum FnParam < 'hir > { Param (& 'hir hir :: Param < 'hir >) , Ident (Option < Ident >) , }
    };
}

FnParam!();