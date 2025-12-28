macro_rules! deps {
    () => {
        Schema!();
        Mutation!();
        Subscription!();
        Query!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < Query , Mutation , Subscription > Clone for Schema < Query , Mutation , Subscription > { fn clone (& self) -> Self { Schema (self . 0 . clone ()) } }
    };
}

impl_125!();