macro_rules! deps {
    () => {
        StringTable!();
        ReadRef!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < 'data , R : ReadRef < 'data > > Default for StringTable < 'data , R > { fn default () -> Self { StringTable { data : None , start : 0 , end : 0 , marker : PhantomData , } } }
    };
}

impl_112!();