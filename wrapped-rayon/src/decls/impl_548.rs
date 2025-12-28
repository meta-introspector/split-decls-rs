macro_rules! deps {
    () => {
        Folder!();
        FlattenIterFolder!();
    };
}

macro_rules! impl_548 {
    () => {
        deps!();
        impl < T , C > Folder < T > for FlattenIterFolder < C > where C : Folder < T :: Item > , T : IntoIterator , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let base = self . base . consume_iter (item) ; FlattenIterFolder { base } } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { let iter = iter . into_iter () . flatten () ; let base = self . base . consume_iter (iter) ; FlattenIterFolder { base } } fn complete (self) -> Self :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_548!();