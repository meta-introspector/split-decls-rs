macro_rules! deps {
    () => {
        ListFolder!();
        Folder!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl < T > Folder < T > for ListFolder < T > { type Result = LinkedList < T > ; fn consume (mut self , item : T) -> Self { self . list . push_back (item) ; self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { self . list . extend (iter) ; self } fn complete (self) -> Self :: Result { self . list } fn full (& self) -> bool { false } }
    };
}

impl_436!();