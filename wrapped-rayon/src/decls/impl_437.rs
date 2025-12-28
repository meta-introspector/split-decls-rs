macro_rules! deps {
    () => {
        Reducer!();
        ListReducer!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl < T > Reducer < LinkedList < T > > for ListReducer { fn reduce (self , mut left : LinkedList < T > , mut right : LinkedList < T >) -> LinkedList < T > { left . append (& mut right) ; left } }
    };
}

impl_437!();