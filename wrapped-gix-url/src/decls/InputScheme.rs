macro_rules! deps {
    () => {
        Url!();
    };
}

macro_rules! InputScheme {
    () => {
        deps!();
        pub (crate) enum InputScheme { Url { protocol_end : usize } , Scp { colon : usize } , Local , }
    };
}

InputScheme!();