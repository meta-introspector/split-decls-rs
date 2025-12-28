macro_rules! deps {
    () => {
        RawIterHashInner!();
        RawIterHash!();
        RawTableInner!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < T > Default for RawIterHash < T > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : unsafe { RawIterHashInner :: new (& RawTableInner :: NEW , 0) } , _marker : PhantomData , } } }
    };
}

impl_109!()