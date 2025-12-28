macro_rules! deps {
    () => {
        TomlLockfileSourceId!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl std :: cmp :: PartialEq for TomlLockfileSourceId { fn eq (& self , other : & Self) -> bool { self . kind == other . kind && self . url == other . url } }
    };
}

impl_53!()