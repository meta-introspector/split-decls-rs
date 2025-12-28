macro_rules! impl_58 {
    () => {
        impl < F1 , F2 > Either < F1 , F2 > { pub (crate) fn left (fut : F1) -> Self { Either :: Left { fut } } pub (crate) fn right (fut : F2) -> Self { Either :: Right { fut } } }
    };
}

impl_58!()