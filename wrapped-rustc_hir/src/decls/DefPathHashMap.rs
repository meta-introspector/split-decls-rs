macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! DefPathHashMap {
    () => {
        deps!();
        pub type DefPathHashMap = odht :: HashTableOwned < Config > ;
    };
}

DefPathHashMap!();