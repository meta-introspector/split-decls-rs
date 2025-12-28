macro_rules! deps {
    () => {
        Child!();
    };
}

macro_rules! impl_639 {
    () => {
        deps!();
        impl < T > Child < T > { fn new (id : T) -> Self { Child { id , children : vec ! [] , } } }
    };
}

impl_639!()