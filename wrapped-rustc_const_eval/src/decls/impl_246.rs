macro_rules! deps {
    () => {
        AllocInfo!();
        AllocKind!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl AllocInfo { fn new (size : Size , align : Align , kind : AllocKind , mutbl : Mutability) -> Self { Self { size , align , kind , mutbl } } }
    };
}

impl_246!();