macro_rules! deps {
    () => {
        VisibleTraits!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl ops :: Deref for VisibleTraits { type Target = FxHashSet < TraitId > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_96!();