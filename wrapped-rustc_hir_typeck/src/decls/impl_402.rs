macro_rules! deps {
    () => {
        Needs!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl Needs { fn maybe_mut_place (m : hir :: Mutability) -> Self { match m { hir :: Mutability :: Mut => Needs :: MutPlace , hir :: Mutability :: Not => Needs :: None , } } }
    };
}

impl_402!();