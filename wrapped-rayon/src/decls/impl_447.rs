macro_rules! deps {
    () => {
        ListStringFolder!();
        Folder!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl Folder < char > for ListStringFolder { type Result = LinkedList < String > ; fn consume (mut self , item : char) -> Self { self . string . push (item) ; self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = char > , { self . string . extend (iter) ; self } fn complete (self) -> Self :: Result { let mut list = LinkedList :: new () ; if ! self . string . is_empty () { list . push_back (self . string) ; } list } fn full (& self) -> bool { false } }
    };
}

impl_447!();