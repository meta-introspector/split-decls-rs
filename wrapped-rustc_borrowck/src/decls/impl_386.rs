macro_rules! deps {
    () => {
        RegionCtxt!();
    };
}

macro_rules! impl_386 {
    () => {
        deps!();
        impl RegionCtxt { # [doc = " Used to determine the representative of a component in the strongly connected"] # [doc = " constraint graph"] pub (crate) fn preference_value (self) -> usize { match self { RegionCtxt :: Unknown => 1 , RegionCtxt :: Existential (None) => 2 , RegionCtxt :: Existential (Some (_)) | RegionCtxt :: Free (_) => 2 , RegionCtxt :: Location (_) => 3 , RegionCtxt :: TyContext (_) => 4 , _ => 5 , } } }
    };
}

impl_386!()