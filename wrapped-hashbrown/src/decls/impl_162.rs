macro_rules! deps {
    () => {
        RawParIter!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < T > Clone for RawParIter < T > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { iter : self . iter . clone () , } } }
    };
}

impl_162!()