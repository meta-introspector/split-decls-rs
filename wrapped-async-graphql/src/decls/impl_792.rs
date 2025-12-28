macro_rules! deps {
    () => {
        MaybeUndefined!();
    };
}

macro_rules! impl_792 {
    () => {
        deps!();
        impl < T > From < Option < Option < T > > > for MaybeUndefined < T > { fn from (value : Option < Option < T > >) -> Self { match value { Some (Some (value)) => Self :: Value (value) , Some (None) => Self :: Null , None => Self :: Undefined , } } }
    };
}

impl_792!()