macro_rules! deps {
    () => {
        DefaultHashBuilder!();
        LinkedHashMap!();
    };
}

macro_rules! LinkedHashSet {
    () => {
        deps!();
        pub struct LinkedHashSet < T , S = DefaultHashBuilder > { map : LinkedHashMap < T , () , S > , }
    };
}

LinkedHashSet!()