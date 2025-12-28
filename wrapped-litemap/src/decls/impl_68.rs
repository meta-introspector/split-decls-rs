macro_rules! deps {
    () => {
        StoreConstEmpty!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'a , K : 'a , V : 'a > StoreConstEmpty < K , V > for & 'a [(K , V)] { const EMPTY : & 'a [(K , V)] = & [] ; }
    };
}

impl_68!()