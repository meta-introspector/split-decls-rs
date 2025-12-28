macro_rules! deps {
    () => {
        FuncLR!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl < L , R , T , F : FnMut (& L , & R) -> T > FuncLR < L , R > for F { type T = T ; }
    };
}

impl_353!()