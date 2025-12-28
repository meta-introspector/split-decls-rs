macro_rules! deps {
    () => {
        FnDecl!();
        FnHeader!();
    };
}

macro_rules! FnSig {
    () => {
        deps!();
        # [doc = " Represents a function's signature in a trait declaration,"] # [doc = " trait implementation, or a free function."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct FnSig < 'hir > { pub header : FnHeader , pub decl : & 'hir FnDecl < 'hir > , pub span : Span , }
    };
}

FnSig!();