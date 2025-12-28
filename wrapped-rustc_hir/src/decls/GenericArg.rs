macro_rules! deps {
    () => {
        Lifetime!();
        TyKind!();
        InferArg!();
        ConstArg!();
        ConstArgKind!();
        Ty!();
        AmbigArg!();
    };
}

macro_rules! GenericArg {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum GenericArg < 'hir > { Lifetime (& 'hir Lifetime) , Type (& 'hir Ty < 'hir , AmbigArg >) , Const (& 'hir ConstArg < 'hir , AmbigArg >) , # [doc = " Inference variables in [`GenericArg`] are always represented by"] # [doc = " `GenericArg::Infer` instead of the `Infer` variants on [`TyKind`] and"] # [doc = " [`ConstArgKind`] as it is not clear until hir ty lowering whether a"] # [doc = " `_` argument is a type or const argument."] # [doc = ""] # [doc = " However, some builtin types' generic arguments are represented by [`TyKind`]"] # [doc = " without a [`GenericArg`], instead directly storing a [`Ty`] or [`ConstArg`]. In"] # [doc = " such cases they *are* represented by the `Infer` variants on [`TyKind`] and"] # [doc = " [`ConstArgKind`] as it is not ambiguous whether the argument is a type or const."] Infer (InferArg) , }
    };
}

GenericArg!();