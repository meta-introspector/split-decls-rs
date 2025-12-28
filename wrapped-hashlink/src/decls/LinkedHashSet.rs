macro_rules! deps {
    () => {
        LinkedHashMap!();
        DefaultHashBuilder!();
    };
}

macro_rules! LinkedHashSet {
    () => {
        deps!();
        pub struct LinkedHashSet < T , S = DefaultHashBuilder > { map : LinkedHashMap < T , () , S > , }
    };
}

LinkedHashSet!();