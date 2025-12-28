macro_rules! deps {
    () => {
        IndexMap!();
        IndexSet!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < T , S , const N : usize > IndexSet < T , BuildHasherDefault < S > , N > { # [doc = " Creates an empty `IndexSet`"] pub const fn new () -> Self { Self { map : IndexMap :: new () , } } }
    };
}

impl_136!();