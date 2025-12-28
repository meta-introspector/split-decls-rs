macro_rules! deps {
    () => {
        Ty!();
        TraitFn!();
        FnPtrTy!();
        Generics!();
        FnSig!();
        Safety!();
    };
}

macro_rules! ForeignItemKind {
    () => {
        deps!();
        # [doc = " An item within an `extern` block."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum ForeignItemKind < 'hir > { # [doc = " A foreign function."] # [doc = ""] # [doc = " All argument idents are actually always present (i.e. `Some`), but"] # [doc = " `&[Option<Ident>]` is used because of code paths shared with `TraitFn`"] # [doc = " and `FnPtrTy`. The sharing is due to all of these cases not allowing"] # [doc = " arbitrary patterns for parameters."] Fn (FnSig < 'hir > , & 'hir [Option < Ident >] , & 'hir Generics < 'hir >) , # [doc = " A foreign static item (`static ext: u8`)."] Static (& 'hir Ty < 'hir > , Mutability , Safety) , # [doc = " A foreign type."] Type , }
    };
}

ForeignItemKind!()