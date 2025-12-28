macro_rules! hashbrown_hashmap {
    () => {
        # [cfg (feature = "hashbrown")] mod hashbrown_hashmap ;
    };
}

hashbrown_hashmap!()