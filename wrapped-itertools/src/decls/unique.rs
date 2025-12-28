macro_rules! deps {
    () => {
        Unique!();
        UniqueBy!();
    };
}

macro_rules! unique {
    () => {
        deps!();
        pub fn unique < I > (iter : I) -> Unique < I > where I : Iterator , I :: Item : Eq + Hash + Clone , { Unique { iter : UniqueBy { iter , used : HashMap :: new () , f : () , } , } }
    };
}

unique!()