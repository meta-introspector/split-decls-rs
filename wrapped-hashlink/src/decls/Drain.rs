macro_rules! Drain {
    () => {
        pub struct Drain < 'a , K : 'a > { iter : linked_hash_map :: Drain < 'a , K , () > , }
    };
}

Drain!()