macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! FnRetTy {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum FnRetTy < 'hir > { # [doc = " Return type is not specified."] # [doc = ""] # [doc = " Functions default to `()` and"] # [doc = " closures default to inference. Span points to where return"] # [doc = " type would be inserted."] DefaultReturn (Span) , # [doc = " Everything else."] Return (& 'hir Ty < 'hir >) , }
    };
}

FnRetTy!();