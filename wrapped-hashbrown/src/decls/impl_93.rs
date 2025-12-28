macro_rules! deps {
    () => {
        RawIntoIter!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < T , A : Allocator > Default for RawIntoIter < T , A > { fn default () -> Self { Self { iter : Default :: default () , allocation : None , marker : PhantomData , } } }
    };
}

impl_93!();