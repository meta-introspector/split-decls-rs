macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! TraitKind {
    () => {
        deps!();
        # [doc (hidden)] pub trait TraitKind : Sized { # [inline] fn anyhow_kind (& self) -> Trait { Trait } }
    };
}

TraitKind!();