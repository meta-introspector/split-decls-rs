macro_rules! deps {
    () => {
        ConstArgKind!();
        AnonConst!();
    };
}

macro_rules! ConstArg {
    () => {
        deps!();
        # [doc = " A constant that enters the type system, used for arguments to const generics (e.g. array lengths)."] # [doc = ""] # [doc = " These are distinct from [`AnonConst`] as anon consts in the type system are not allowed"] # [doc = " to use any generic parameters, therefore we must represent `N` differently. Additionally"] # [doc = " future designs for supporting generic parameters in const arguments will likely not use"] # [doc = " an anon const based design."] # [doc = ""] # [doc = " So, `ConstArg` (specifically, [`ConstArgKind`]) distinguishes between const args"] # [doc = " that are [just paths](ConstArgKind::Path) (currently just bare const params)"] # [doc = " versus const args that are literals or have arbitrary computations (e.g., `{ 1 + 3 }`)."] # [doc = ""] # [doc = " For an explanation of the `Unambig` generic parameter see the dev-guide:"] # [doc = " <https://rustc-dev-guide.rust-lang.org/hir/ambig-unambig-ty-and-consts.html>"] # [derive (Clone , Copy , Debug , HashStable_Generic)] # [repr (C)] pub struct ConstArg < 'hir , Unambig = () > { # [stable_hasher (ignore)] pub hir_id : HirId , pub kind : ConstArgKind < 'hir , Unambig > , }
    };
}

ConstArg!();