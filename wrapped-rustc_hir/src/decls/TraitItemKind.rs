macro_rules! deps {
    () => {
        TraitFn!();
        GenericBounds!();
        FnSig!();
        Ty!();
        BodyId!();
    };
}

macro_rules! TraitItemKind {
    () => {
        deps!();
        # [doc = " Represents a trait method or associated constant or type"] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum TraitItemKind < 'hir > { # [doc = " An associated constant with an optional value (otherwise `impl`s must contain a value)."] Const (& 'hir Ty < 'hir > , Option < BodyId >) , # [doc = " An associated function with an optional body."] Fn (FnSig < 'hir > , TraitFn < 'hir >) , # [doc = " An associated type with (possibly empty) bounds and optional concrete"] # [doc = " type."] Type (GenericBounds < 'hir > , Option < & 'hir Ty < 'hir > >) , }
    };
}

TraitItemKind!()