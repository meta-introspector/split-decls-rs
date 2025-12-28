macro_rules! deps {
    () => {
        Topo!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < N , VM > Default for Topo < N , VM > where VM : Default , { fn default () -> Self { Topo { tovisit : Vec :: new () , ordered : VM :: default () , } } }
    };
}

impl_52!();