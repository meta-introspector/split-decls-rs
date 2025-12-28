macro_rules! deps {
    () => {
        UnionFind!();
    };
}

macro_rules! impl_1062 {
    () => {
        deps!();
        impl < K > Default for UnionFind < K > { fn default () -> Self { Self { parent : Vec :: new () , rank : Vec :: new () , } } }
    };
}

impl_1062!()