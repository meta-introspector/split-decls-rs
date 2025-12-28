macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < T , S , const N : usize > Default for IndexSet < T , S , N > where S : Default , { fn default () -> Self { Self { map : < _ > :: default () , } } }
    };
}

impl_141!();