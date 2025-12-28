macro_rules! deps {
    () => {
        InvisibleOrigin!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl InvisibleOrigin { # [inline] pub fn skip (& self) -> bool { match self { InvisibleOrigin :: MetaVar (_) => false , InvisibleOrigin :: ProcMacro => true , } } }
    };
}

impl_393!();