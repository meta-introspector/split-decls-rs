macro_rules! deps {
    () => {
        Compat!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T > Compat < T > { pub (crate) fn new (io : T) -> Self { Compat (io) } fn p (self : Pin < & mut Self >) -> Pin < & mut T > { unsafe { self . map_unchecked_mut (| me | & mut me . 0) } } }
    };
}

impl_67!()