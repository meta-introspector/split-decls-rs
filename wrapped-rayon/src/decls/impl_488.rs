macro_rules! deps {
    () => {
        Reducer!();
        FindReducer!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl < T > Reducer < Option < T > > for FindReducer { fn reduce (self , left : Option < T > , right : Option < T >) -> Option < T > { left . or (right) } }
    };
}

impl_488!();