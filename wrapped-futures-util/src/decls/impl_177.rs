macro_rules! impl_177 {
    () => {
        impl < T > From < Option < T > > for OptionFuture < T > { fn from (option : Option < T >) -> Self { Self { inner : option } } }
    };
}

impl_177!()