macro_rules! deps {
    () => {
        RawIter!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < T > Clone for RawIter < T > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { iter : self . iter . clone () , items : self . items , } } }
    };
}

impl_77!()