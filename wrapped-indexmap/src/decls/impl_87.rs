macro_rules! deps {
    () => {
        IndexSet!();
        IndexMap!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < T , S > Default for IndexSet < T , S > where S : Default , { # [doc = " Return an empty [`IndexSet`]"] fn default () -> Self { IndexSet { map : IndexMap :: default () , } } }
    };
}

impl_87!();