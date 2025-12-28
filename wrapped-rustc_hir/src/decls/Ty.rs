macro_rules! deps {
    () => {
        TyKind!();
    };
}

macro_rules! Ty {
    () => {
        deps!();
        # [doc = " Represents a type in the `HIR`."] # [doc = ""] # [doc = " For an explanation of the `Unambig` generic parameter see the dev-guide:"] # [doc = " <https://rustc-dev-guide.rust-lang.org/hir/ambig-unambig-ty-and-consts.html>"] # [derive (Debug , Clone , Copy , HashStable_Generic)] # [repr (C)] pub struct Ty < 'hir , Unambig = () > { # [stable_hasher (ignore)] pub hir_id : HirId , pub span : Span , pub kind : TyKind < 'hir , Unambig > , }
    };
}

Ty!()