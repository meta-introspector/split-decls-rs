macro_rules! deps {
    () => {
        TomlLockfileSourceId!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Ord for TomlLockfileSourceId { fn cmp (& self , other : & TomlLockfileSourceId) -> Ordering { self . kind . cmp (& other . kind) . then_with (| | self . url . cmp (& other . url)) } }
    };
}

impl_56!();