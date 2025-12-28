macro_rules! deps {
    () => {
        FnRetTy!();
        Ty!();
        ImplicitSelfKind!();
    };
}

macro_rules! FnDecl {
    () => {
        deps!();
        # [doc = " Represents the header (not the body) of a function declaration."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct FnDecl < 'hir > { # [doc = " The types of the function's parameters."] # [doc = ""] # [doc = " Additional argument data is stored in the function's [body](Body::params)."] pub inputs : & 'hir [Ty < 'hir >] , pub output : FnRetTy < 'hir > , pub c_variadic : bool , # [doc = " Does the function have an implicit self?"] pub implicit_self : ImplicitSelfKind , # [doc = " Is lifetime elision allowed."] pub lifetime_elision_allowed : bool , }
    };
}

FnDecl!();