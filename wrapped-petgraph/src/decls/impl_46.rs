macro_rules! deps {
    () => {
        DfsPostOrder!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < N , VM > Default for DfsPostOrder < N , VM > where VM : Default , { fn default () -> Self { DfsPostOrder { stack : Vec :: new () , discovered : VM :: default () , finished : VM :: default () , } } }
    };
}

impl_46!();