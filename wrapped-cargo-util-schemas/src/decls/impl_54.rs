macro_rules! deps {
    () => {
        TomlLockfileSourceId!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl std :: cmp :: Eq for TomlLockfileSourceId { }
    };
}

impl_54!()