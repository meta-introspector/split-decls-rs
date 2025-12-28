macro_rules! deps {
    () => {
        Bucket!();
    };
}

macro_rules! HashTable {
    () => {
        deps!();
        struct HashTable { entries : Box < [Bucket] > , hash_bits : u32 , _prev : * const HashTable , }
    };
}

HashTable!()