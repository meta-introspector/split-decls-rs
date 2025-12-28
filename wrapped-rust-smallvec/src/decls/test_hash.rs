macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_hash {
    () => {
        deps!();
        # [test] fn test_hash () { use std :: collections :: hash_map :: DefaultHasher ; use std :: hash :: Hash ; fn hash (value : impl Hash) -> u64 { let mut hasher = DefaultHasher :: new () ; value . hash (& mut hasher) ; hasher . finish () } { let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; let b = [1 , 2] ; a . extend (b . iter () . cloned ()) ; assert_eq ! (hash (a) , hash (b)) ; } { let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; let b = [1 , 2 , 11 , 12] ; a . extend (b . iter () . cloned ()) ; assert_eq ! (hash (a) , hash (b)) ; } }
    };
}

test_hash!();