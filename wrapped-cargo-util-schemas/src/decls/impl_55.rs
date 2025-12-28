macro_rules! deps {
    () => {
        TomlLockfileSourceId!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl PartialOrd for TomlLockfileSourceId { fn partial_cmp (& self , other : & TomlLockfileSourceId) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_55!();