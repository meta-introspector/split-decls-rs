macro_rules! deps {
    () => {
        MultiProductIter!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < I > MultiProductIter < I > where I : Iterator + Clone , I :: Item : Clone , { fn new (iter : I) -> Self { Self { iter : iter . clone () , iter_orig : iter , } } }
    };
}

impl_66!();