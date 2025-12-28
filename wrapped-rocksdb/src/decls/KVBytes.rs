macro_rules! KVBytes {
    () => {
        pub type KVBytes = (Box < [u8] > , Box < [u8] >) ;
    };
}

KVBytes!();